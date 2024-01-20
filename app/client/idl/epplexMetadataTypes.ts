export type EpplexMetadata = {
  "version": "0.1.0",
  "name": "epplex_metadata",
  "instructions": [
    {
      "name": "createMetadata",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
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
          "isMut": true,
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
            "defined": "CreateMetadataParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "tokenMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "updateAuthority",
            "docs": [
              "The authority that can sign to update the metadata"
            ],
            "type": "publicKey"
          },
          {
            "name": "mint",
            "docs": [
              "The associated mint, used to counter spoofing to be sure that metadata",
              "belongs to a particular mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "name",
            "docs": [
              "The longer name of the token"
            ],
            "type": "string"
          },
          {
            "name": "symbol",
            "docs": [
              "The shortened symbol for the token"
            ],
            "type": "string"
          },
          {
            "name": "uri",
            "docs": [
              "The URI pointing to richer metadata"
            ],
            "type": "string"
          },
          {
            "name": "additionalMetadata",
            "docs": [
              "Any additional metadata about the token as key-value pairs. The program",
              "must avoid storing the same key twice."
            ],
            "type": {
              "vec": {
                "array": [
                  "string",
                  2
                ]
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateMetadataParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destroyTimestamp",
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
    }
  ]
};

export const IDL: EpplexMetadata = {
  "version": "0.1.0",
  "name": "epplex_metadata",
  "instructions": [
    {
      "name": "createMetadata",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
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
          "isMut": true,
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
            "defined": "CreateMetadataParams"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "tokenMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "updateAuthority",
            "docs": [
              "The authority that can sign to update the metadata"
            ],
            "type": "publicKey"
          },
          {
            "name": "mint",
            "docs": [
              "The associated mint, used to counter spoofing to be sure that metadata",
              "belongs to a particular mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "name",
            "docs": [
              "The longer name of the token"
            ],
            "type": "string"
          },
          {
            "name": "symbol",
            "docs": [
              "The shortened symbol for the token"
            ],
            "type": "string"
          },
          {
            "name": "uri",
            "docs": [
              "The URI pointing to richer metadata"
            ],
            "type": "string"
          },
          {
            "name": "additionalMetadata",
            "docs": [
              "Any additional metadata about the token as key-value pairs. The program",
              "must avoid storing the same key twice."
            ],
            "type": {
              "vec": {
                "array": [
                  "string",
                  2
                ]
              }
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "CreateMetadataParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destroyTimestamp",
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
    }
  ]
};
