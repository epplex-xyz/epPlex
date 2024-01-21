export type EpplexCore = {
  "version": "0.1.0",
  "name": "epplex_core",
  "instructions": [
    {
      "name": "tokenCreate",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK inside CPI"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "tokenMint",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenMetadata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
        },
        {
          "name": "associatedToken",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "collectionCreate",
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
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collectionConfig",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "globalCollectionConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "CollectionCreateParams"
          }
        }
      ]
    },
    {
      "name": "collectionMint",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenMetadata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "collectionConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintAuthority",
          "isMut": false,
          "isSigner": true
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
        },
        {
          "name": "associatedToken",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "globalCollectionConfigCreate",
      "accounts": [
        {
          "name": "globalCollectionConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "programDelegateCreate",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCreateParams"
          }
        }
      ]
    },
    {
      "name": "programDelegateClose",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCloseParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collectionConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "The bump, used for PDA validation."
            ],
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "renewalPrice",
            "type": "u64"
          },
          {
            "name": "mintPrice",
            "type": "u64"
          },
          {
            "name": "standardDuration",
            "type": "u32"
          },
          {
            "name": "gracePeriod",
            "type": "i64"
          },
          {
            "name": "treasury",
            "type": "publicKey"
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
      "name": "globalCollectionConfig",
      "type": {
        "kind": "struct",
        "fields": [
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
    },
    {
      "name": "programDelegate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "The bump, used for PDA validation."
            ],
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CollectionCreateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "renewalPrice",
            "type": "u64"
          },
          {
            "name": "mintPrice",
            "type": "u64"
          },
          {
            "name": "standardDuration",
            "type": "u32"
          },
          {
            "name": "gracePeriod",
            "type": "i64"
          },
          {
            "name": "treasury",
            "type": "publicKey"
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
      "name": "ProgramDelegateCloseParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "ProgramDelegateCreateParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenCreateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destroyTimestampOffset",
            "type": "i64"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "MintError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "UnauthorizedMintAuthority"
          },
          {
            "name": "InvalidTreasuryAccount"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidCalculation",
      "msg": "Invalid calculation"
    },
    {
      "code": 6001,
      "name": "DestroyTimestampNotExceeded",
      "msg": "Destroy timestamp has not been exceeded"
    }
  ]
};

export const IDL: EpplexCore = {
  "version": "0.1.0",
  "name": "epplex_core",
  "instructions": [
    {
      "name": "tokenCreate",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenMetadata",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK inside CPI"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "tokenMint",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenMetadata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
        },
        {
          "name": "associatedToken",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "collectionCreate",
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
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "collectionConfig",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "globalCollectionConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "token22Program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "CollectionCreateParams"
          }
        }
      ]
    },
    {
      "name": "collectionMint",
      "accounts": [
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "ata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "tokenMetadata",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "collectionConfig",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "treasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "programDelegate",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintAuthority",
          "isMut": false,
          "isSigner": true
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
        },
        {
          "name": "associatedToken",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadataProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "TokenCreateParams"
          }
        }
      ]
    },
    {
      "name": "globalCollectionConfigCreate",
      "accounts": [
        {
          "name": "globalCollectionConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "programDelegateCreate",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCreateParams"
          }
        }
      ]
    },
    {
      "name": "programDelegateClose",
      "accounts": [
        {
          "name": "programDelegate",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "ProgramDelegateCloseParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "collectionConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "The bump, used for PDA validation."
            ],
            "type": "u8"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "renewalPrice",
            "type": "u64"
          },
          {
            "name": "mintPrice",
            "type": "u64"
          },
          {
            "name": "standardDuration",
            "type": "u32"
          },
          {
            "name": "gracePeriod",
            "type": "i64"
          },
          {
            "name": "treasury",
            "type": "publicKey"
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
      "name": "globalCollectionConfig",
      "type": {
        "kind": "struct",
        "fields": [
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
    },
    {
      "name": "programDelegate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "docs": [
              "The bump, used for PDA validation."
            ],
            "type": "u8"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CollectionCreateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "renewalPrice",
            "type": "u64"
          },
          {
            "name": "mintPrice",
            "type": "u64"
          },
          {
            "name": "standardDuration",
            "type": "u32"
          },
          {
            "name": "gracePeriod",
            "type": "i64"
          },
          {
            "name": "treasury",
            "type": "publicKey"
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
      "name": "ProgramDelegateCloseParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "ProgramDelegateCreateParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "TokenCreateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destroyTimestampOffset",
            "type": "i64"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "MintError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "UnauthorizedMintAuthority"
          },
          {
            "name": "InvalidTreasuryAccount"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidCalculation",
      "msg": "Invalid calculation"
    },
    {
      "code": 6001,
      "name": "DestroyTimestampNotExceeded",
      "msg": "Destroy timestamp has not been exceeded"
    }
  ]
};
