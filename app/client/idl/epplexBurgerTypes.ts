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
          "name": "mint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ataBuyer",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "ataSeller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
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
    },
    {
      "name": "tokenRenew",
      "accounts": [
        {
          "name": "mintPayment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proceedsTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenRenewParams"
          }
        }
      ]
    },
    {
      "name": "voteCast",
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
            "defined": "VoteCastParams"
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
      "name": "VoteCastParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "message",
            "type": "string"
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "DestroyTimestampHasBeenExceeded",
      "msg": "Destroy timestamp has been exceeded"
    },
    {
      "code": 6001,
      "name": "TokenNotSupported",
      "msg": "Token not supported"
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
          "name": "mint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "ataBuyer",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "ataSeller",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
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
    },
    {
      "name": "tokenRenew",
      "accounts": [
        {
          "name": "mintPayment",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "proceedsTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "authority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenRenewParams"
          }
        }
      ]
    },
    {
      "name": "voteCast",
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
            "defined": "VoteCastParams"
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
      "name": "VoteCastParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "message",
            "type": "string"
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "DestroyTimestampHasBeenExceeded",
      "msg": "Destroy timestamp has been exceeded"
    },
    {
      "code": 6001,
      "name": "TokenNotSupported",
      "msg": "Token not supported"
    }
  ]
};
