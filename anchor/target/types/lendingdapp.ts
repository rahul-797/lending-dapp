/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/lendingdapp.json`.
 */
export type Lendingdapp = {
  "address": "Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe",
  "metadata": {
    "name": "lendingdapp",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initBank",
      "discriminator": [
        73,
        111,
        27,
        243,
        202,
        129,
        159,
        80
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "mintAddress"
        },
        {
          "name": "bank",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  97,
                  110,
                  107
                ]
              },
              {
                "kind": "account",
                "path": "mintAddress"
              }
            ]
          }
        },
        {
          "name": "bankVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  97,
                  110,
                  107,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "mintAddress"
              }
            ]
          }
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initUser",
      "discriminator": [
        14,
        51,
        68,
        159,
        237,
        78,
        158,
        102
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "bank",
      "discriminator": [
        142,
        49,
        166,
        242,
        50,
        66,
        97,
        188
      ]
    },
    {
      "name": "user",
      "discriminator": [
        159,
        117,
        95,
        227,
        239,
        151,
        58,
        236
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "insufficientFunds",
      "msg": "Insufficient funds in bank vault."
    },
    {
      "code": 6001,
      "name": "userNotInitialized",
      "msg": "User account is not initialized."
    },
    {
      "code": 6002,
      "name": "bankNotInitialized",
      "msg": "Bank account is not initialized."
    },
    {
      "code": 6003,
      "name": "invalidMint",
      "msg": "Invalid mint provided."
    }
  ],
  "types": [
    {
      "name": "bank",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "mintAddress",
            "type": "pubkey"
          },
          {
            "name": "totalDeposits",
            "type": "u64"
          },
          {
            "name": "totalDepositShares",
            "type": "u64"
          },
          {
            "name": "totalBorrowed",
            "type": "u64"
          },
          {
            "name": "totalBorrowedShares",
            "type": "u64"
          },
          {
            "name": "liquidationThreshold",
            "type": "u64"
          },
          {
            "name": "liquidationBonus",
            "type": "u64"
          },
          {
            "name": "liquidationCloseFactor",
            "type": "u64"
          },
          {
            "name": "maxLtv",
            "type": "u64"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          },
          {
            "name": "interestRate",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "user",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "depositedSol",
            "type": "u64"
          },
          {
            "name": "depositedSolShares",
            "type": "u64"
          },
          {
            "name": "borrowedSol",
            "type": "u64"
          },
          {
            "name": "borrowedSolShares",
            "type": "u64"
          },
          {
            "name": "depositedUsdc",
            "type": "u64"
          },
          {
            "name": "depositedUsdcShares",
            "type": "u64"
          },
          {
            "name": "borrowedUsdc",
            "type": "u64"
          },
          {
            "name": "borrowedUsdcShares",
            "type": "u64"
          },
          {
            "name": "usdcAddress",
            "type": "pubkey"
          },
          {
            "name": "healthFactor",
            "type": "u64"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          }
        ]
      }
    }
  ]
};
