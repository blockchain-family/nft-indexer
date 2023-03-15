Force refresh metadata

```
POST /metadata/refresh/

# for one NFT
{
    "nft" : "0:bbe069479f784b51b8818e624c3254b003bad14bb9b1787593c187100e4b361c",
    "collection": "0:4876694042b5b385318f2bd49f2eebf9d68913f1ccd723ab95c5ccb12979c8ba"
}

# for all nfts in collection
{
    "collection": "0:4876694042b5b385318f2bd49f2eebf9d68913f1ccd723ab95c5ccb12979c8ba"
}
```