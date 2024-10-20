DO $$
DECLARE
    batch_size INT := 100000;
    updated_rows INT := 0;
BEGIN
    LOOP
        WITH limited_rows AS (
            SELECT n.address
            FROM nft n
            WHERE n.metadata_updated_at IS NULL
            LIMIT batch_size
        )
        UPDATE nft n
        SET metadata_updated_at = m.updated_at
        FROM meta_handled_addresses m, limited_rows lr
        WHERE m.address = n.address AND NOT m.failed AND n.address = lr.address;

        GET DIAGNOSTICS updated_rows = ROW_COUNT;
        RAISE NOTICE 'Updated rows: %', updated_rows;

        IF updated_rows = 0 THEN
            EXIT;
        END IF;
    END LOOP;
END $$;


create table backup_mha_failed as
select * from meta_handled_addresses
where failed;

truncate table meta_handled_addresses;

insert into meta_handled_addresses
select * from backup_mha_failed;

drop table backup_mha_failed;