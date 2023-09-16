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
          "isSigner": true,
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
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
    }
  ],
  "accounts": [
    {
      "name": "token",
      "docs": [
        "The primary lotto account"
      ],
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
      "name": "TokenCreateParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
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
          "isSigner": true,
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
          "name": "payer",
          "isMut": true,
          "isSigner": true
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
    }
  ],
  "accounts": [
    {
      "name": "token",
      "docs": [
        "The primary lotto account"
      ],
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
      "name": "TokenCreateParams",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ]
};
