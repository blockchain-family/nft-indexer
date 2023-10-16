CREATE OR REPLACE FUNCTION safe_refresh_nft_verified_extended(p_flag boolean) RETURNS void AS
$$
DECLARE
    v_lock_key CONSTANT INTEGER := 123456;
    v_got_lock BOOLEAN;
BEGIN
    v_got_lock := pg_try_advisory_lock(v_lock_key);

    IF NOT v_got_lock THEN
        RAISE NOTICE 'Another instance is running. Exiting...';
        RETURN;
    END IF;
    PERFORM refresh_nft_verified_extended(p_flag);
    PERFORM pg_advisory_unlock(v_lock_key);
END;
$$ LANGUAGE plpgsql;
