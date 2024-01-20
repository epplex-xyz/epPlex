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
    },
    {
      "name": "tokenBuy",
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
            "defined": "TokenBuyParams"
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
      "name": "TokenBurnParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenBuyParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenRenewParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "renewTerms",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "WhitelistMintParams",
      "type": {
        "kind": "struct",
        "fields": []
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
    },
    {
      "name": "tokenBuy",
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
            "defined": "TokenBuyParams"
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
      "name": "TokenBurnParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenBuyParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenRenewParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "renewTerms",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "WhitelistMintParams",
      "type": {
        "kind": "struct",
        "fields": []
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
