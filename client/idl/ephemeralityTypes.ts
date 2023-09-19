export type Ephemerality = {
  "version": "0.0.1",
  "name": "ephemerality",
  "instructions": [
    {
      "name": "tokenCreate",
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
      "name": "tokenBurn",
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
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
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
            "defined": "TokenBurnParams"
          }
        }
      ]
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
      "name": "TokenBurnParams",
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
      "name": "Metadata",
      "type": {
        "kind": "struct",
        "fields": [
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
          },
          {
            "name": "dunno7",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "destroyTimestampField",
            "type": "string"
          },
          {
            "name": "destroyTimestampValue",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "Mint22",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintAuthority",
            "docs": [
              "Optional authority used to mint new tokens. The mint authority may only be provided during",
              "mint creation. If no mint authority is present then the mint has a fixed supply and no",
              "further tokens may be minted."
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "supply",
            "docs": [
              "Total supply of tokens."
            ],
            "type": "u64"
          },
          {
            "name": "decimals",
            "docs": [
              "Number of base 10 digits to the right of the decimal place."
            ],
            "type": "u8"
          },
          {
            "name": "isInitialized",
            "docs": [
              "Is `true` if this structure has been initialized"
            ],
            "type": "bool"
          },
          {
            "name": "freezeAuthority",
            "docs": [
              "Optional authority to freeze token accounts."
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                83
              ]
            }
          },
          {
            "name": "dunno1",
            "type": {
              "array": [
                "u8",
                5
              ]
            }
          },
          {
            "name": "closeAuthority",
            "type": "publicKey"
          },
          {
            "name": "dunno2",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "permanentDelegate",
            "type": "publicKey"
          },
          {
            "name": "dunno3",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "dunno4",
            "type": "publicKey"
          },
          {
            "name": "dunno5",
            "type": "publicKey"
          },
          {
            "name": "dunno6",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "metadataPointerAuthority",
            "type": "publicKey"
          },
          {
            "name": "metadataAddress",
            "type": "publicKey"
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
          },
          {
            "name": "dunno7",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "destroyTimestampField",
            "type": "string"
          },
          {
            "name": "destroyTimestampValue",
            "type": "string"
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

export const IDL: Ephemerality = {
  "version": "0.0.1",
  "name": "ephemerality",
  "instructions": [
    {
      "name": "tokenCreate",
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
      "name": "tokenBurn",
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
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
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
            "defined": "TokenBurnParams"
          }
        }
      ]
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
      "name": "TokenBurnParams",
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
      "name": "Metadata",
      "type": {
        "kind": "struct",
        "fields": [
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
          },
          {
            "name": "dunno7",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "destroyTimestampField",
            "type": "string"
          },
          {
            "name": "destroyTimestampValue",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "Mint22",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintAuthority",
            "docs": [
              "Optional authority used to mint new tokens. The mint authority may only be provided during",
              "mint creation. If no mint authority is present then the mint has a fixed supply and no",
              "further tokens may be minted."
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "supply",
            "docs": [
              "Total supply of tokens."
            ],
            "type": "u64"
          },
          {
            "name": "decimals",
            "docs": [
              "Number of base 10 digits to the right of the decimal place."
            ],
            "type": "u8"
          },
          {
            "name": "isInitialized",
            "docs": [
              "Is `true` if this structure has been initialized"
            ],
            "type": "bool"
          },
          {
            "name": "freezeAuthority",
            "docs": [
              "Optional authority to freeze token accounts."
            ],
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                83
              ]
            }
          },
          {
            "name": "dunno1",
            "type": {
              "array": [
                "u8",
                5
              ]
            }
          },
          {
            "name": "closeAuthority",
            "type": "publicKey"
          },
          {
            "name": "dunno2",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "permanentDelegate",
            "type": "publicKey"
          },
          {
            "name": "dunno3",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "dunno4",
            "type": "publicKey"
          },
          {
            "name": "dunno5",
            "type": "publicKey"
          },
          {
            "name": "dunno6",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "metadataPointerAuthority",
            "type": "publicKey"
          },
          {
            "name": "metadataAddress",
            "type": "publicKey"
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
          },
          {
            "name": "dunno7",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "destroyTimestampField",
            "type": "string"
          },
          {
            "name": "destroyTimestampValue",
            "type": "string"
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
