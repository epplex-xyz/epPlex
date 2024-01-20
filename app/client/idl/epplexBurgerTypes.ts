export type EpplexBurger = {
  "version": "0.1.0",
  "name": "epplex_burger",
  "instructions": [
    {
      "name": "whitelistMint",
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "WhitelistMintParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "mintGuard",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "itemsMinted",
            "type": "u32"
          },
          {
            "name": "collectionCounter",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "WhitelistMintParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collectionRenewalPrice",
            "type": "u64"
          },
          {
            "name": "collectionMintPrice",
            "type": "u64"
          },
          {
            "name": "collectionStandardDuration",
            "type": "u32"
          },
          {
            "name": "collectionGracePeriod",
            "type": "i64"
          },
          {
            "name": "collectionSize",
            "type": "u32"
          },
          {
            "name": "collectionName",
            "type": "string"
          },
          {
            "name": "collectionSymbol",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "WithdrawError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidAuthority"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "CollectionMintedOut",
      "msg": "Collection already minted out"
    }
  ]
};

export const IDL: EpplexBurger = {
  "version": "0.1.0",
  "name": "epplex_burger",
  "instructions": [
    {
      "name": "whitelistMint",
      "accounts": [
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "WhitelistMintParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "mintGuard",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "itemsMinted",
            "type": "u32"
          },
          {
            "name": "collectionCounter",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "WhitelistMintParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "collectionRenewalPrice",
            "type": "u64"
          },
          {
            "name": "collectionMintPrice",
            "type": "u64"
          },
          {
            "name": "collectionStandardDuration",
            "type": "u32"
          },
          {
            "name": "collectionGracePeriod",
            "type": "i64"
          },
          {
            "name": "collectionSize",
            "type": "u32"
          },
          {
            "name": "collectionName",
            "type": "string"
          },
          {
            "name": "collectionSymbol",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "WithdrawError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "InvalidAuthority"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "CollectionMintedOut",
      "msg": "Collection already minted out"
    }
  ]
};
