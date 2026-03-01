/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/inco_token.json`.
 */
export type IncoToken = {
  "address": "8kKZoqm42xJtu1JWvH1ZeoLsucVyUKpGfmhrY2eBHjBK",
  "metadata": {
    "name": "incoToken",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Private Vault - Encrypted Vault Program for Solana using Inco Lightning"
  },
  "instructions": [
    {
      "name": "addExtraCondition",
      "docs": [
        "Add an extra condition to a vault"
      ],
      "discriminator": [
        112,
        127,
        220,
        244,
        3,
        132,
        137,
        111
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "extraCondition",
          "writable": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "condition",
          "type": {
            "defined": {
              "name": "conditionInput"
            }
          }
        }
      ]
    },
    {
      "name": "addObserver",
      "docs": [
        "Add an observer to a vault's observer list"
      ],
      "discriminator": [
        233,
        72,
        76,
        49,
        240,
        143,
        134,
        53
      ],
      "accounts": [
        {
          "name": "vault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "observerList",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  111,
                  98,
                  115,
                  101,
                  114,
                  118,
                  101,
                  114,
                  115
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "observer",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "addSecretChunk",
      "docs": [
        "Add encrypted secret chunk to a vault",
        "remaining_accounts: [vault_allowance, vault_address, creator_allowance, creator_address]"
      ],
      "discriminator": [
        129,
        113,
        122,
        192,
        171,
        145,
        189,
        189
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "secretChunk",
          "writable": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "ciphertext",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "claimVault",
      "docs": [
        "Claim vault assets (verify recipient + check conditions)"
      ],
      "discriminator": [
        87,
        111,
        176,
        185,
        53,
        172,
        227,
        137
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "claimer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "claimSalt",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "conditionSalt",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "conditionValues",
          "type": {
            "vec": "u64"
          }
        }
      ]
    },
    {
      "name": "createVault",
      "docs": [
        "Create a vault with SOL deposit + primary condition"
      ],
      "discriminator": [
        29,
        237,
        247,
        208,
        193,
        82,
        54,
        135
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "args.vault_id"
              }
            ]
          }
        },
        {
          "name": "vaultCounter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "createVaultArgs"
            }
          }
        }
      ]
    },
    {
      "name": "depositForCondition",
      "docs": [
        "Deposit SOL for IncomingTransaction condition trigger"
      ],
      "discriminator": [
        240,
        100,
        208,
        144,
        181,
        244,
        143,
        86
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "depositTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  100,
                  101,
                  112,
                  111,
                  115,
                  105,
                  116,
                  115
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              },
              {
                "kind": "const",
                "value": [
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0,
                  0
                ]
              }
            ]
          }
        },
        {
          "name": "activityTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  97,
                  115,
                  116,
                  95,
                  97,
                  99,
                  116,
                  105,
                  118,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "depositor"
              }
            ]
          }
        },
        {
          "name": "depositor",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "grantDecryptionRights",
      "docs": [
        "Grant FHE decryption rights (for secret vaults after conditions met)",
        "remaining_accounts: ExtraCondition PDAs + condition data + FHE [allowance, address] pairs"
      ],
      "discriminator": [
        89,
        151,
        122,
        166,
        76,
        90,
        27,
        172
      ],
      "accounts": [
        {
          "name": "vault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "caller",
          "writable": true,
          "signer": true
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "claimSalt",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "conditionSalt",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "conditionValues",
          "type": {
            "vec": "u64"
          }
        }
      ]
    },
    {
      "name": "initializeVaultCounter",
      "docs": [
        "Initialize the global vault counter (one-time admin setup)"
      ],
      "discriminator": [
        101,
        77,
        148,
        61,
        230,
        129,
        130,
        231
      ],
      "accounts": [
        {
          "name": "vaultCounter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  99,
                  111,
                  117,
                  110,
                  116,
                  101,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "recordActivity",
      "docs": [
        "Record wallet activity (resets inactivity timer)"
      ],
      "discriminator": [
        199,
        86,
        104,
        65,
        200,
        211,
        71,
        50
      ],
      "accounts": [
        {
          "name": "activityTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  97,
                  115,
                  116,
                  95,
                  97,
                  99,
                  116,
                  105,
                  118,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "recordActivityBySig",
      "docs": [
        "Record wallet activity via Ed25519 signature (relayer pattern)"
      ],
      "discriminator": [
        248,
        212,
        140,
        222,
        81,
        167,
        199,
        195
      ],
      "accounts": [
        {
          "name": "activityTracker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  97,
                  115,
                  116,
                  95,
                  97,
                  99,
                  116,
                  105,
                  118,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "arg",
                "path": "wallet"
              }
            ]
          }
        },
        {
          "name": "activityNonce",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  99,
                  116,
                  105,
                  118,
                  105,
                  116,
                  121,
                  95,
                  110,
                  111,
                  110,
                  99,
                  101
                ]
              },
              {
                "kind": "arg",
                "path": "wallet"
              }
            ]
          }
        },
        {
          "name": "relayer",
          "writable": true,
          "signer": true
        },
        {
          "name": "instructionSysvar",
          "address": "Sysvar1nstructions1111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "wallet",
          "type": "pubkey"
        },
        {
          "name": "nonce",
          "type": "u64"
        },
        {
          "name": "deadline",
          "type": "i64"
        }
      ]
    },
    {
      "name": "refundVault",
      "docs": [
        "Refund vault after deadline",
        "remaining_accounts (SPL): [vault_token_account, caller_token_account, token_program]"
      ],
      "discriminator": [
        229,
        254,
        53,
        165,
        5,
        201,
        46,
        54
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "caller",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "refundSalt",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "setEncryptedField",
      "docs": [
        "Add encrypted field to an existing vault (FHE)",
        "remaining_accounts: [vault_allowance, vault_address, creator_allowance, creator_address]"
      ],
      "discriminator": [
        115,
        99,
        214,
        235,
        153,
        1,
        189,
        102
      ],
      "accounts": [
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "vaultId"
              }
            ]
          }
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "vaultId",
          "type": "u64"
        },
        {
          "name": "fieldType",
          "type": "u8"
        },
        {
          "name": "ciphertext",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "activityNonce",
      "discriminator": [
        129,
        104,
        216,
        142,
        243,
        200,
        227,
        245
      ]
    },
    {
      "name": "activityTracker",
      "discriminator": [
        115,
        180,
        99,
        189,
        78,
        165,
        129,
        164
      ]
    },
    {
      "name": "conditionDepositTracker",
      "discriminator": [
        144,
        112,
        61,
        90,
        118,
        204,
        219,
        139
      ]
    },
    {
      "name": "extraCondition",
      "discriminator": [
        20,
        58,
        139,
        224,
        168,
        31,
        1,
        78
      ]
    },
    {
      "name": "observerList",
      "discriminator": [
        227,
        143,
        130,
        136,
        246,
        187,
        76,
        174
      ]
    },
    {
      "name": "secretChunk",
      "discriminator": [
        67,
        110,
        179,
        38,
        67,
        13,
        130,
        83
      ]
    },
    {
      "name": "vault",
      "discriminator": [
        211,
        8,
        232,
        43,
        2,
        152,
        117,
        119
      ]
    },
    {
      "name": "vaultCounter",
      "discriminator": [
        180,
        127,
        122,
        230,
        154,
        126,
        126,
        98
      ]
    }
  ],
  "events": [
    {
      "name": "activityRecorded",
      "discriminator": [
        13,
        102,
        161,
        232,
        11,
        141,
        84,
        217
      ]
    },
    {
      "name": "conditionDeposited",
      "discriminator": [
        224,
        132,
        165,
        121,
        240,
        116,
        163,
        183
      ]
    },
    {
      "name": "encryptedFieldSet",
      "discriminator": [
        162,
        125,
        81,
        131,
        129,
        24,
        194,
        172
      ]
    },
    {
      "name": "extraConditionAdded",
      "discriminator": [
        15,
        26,
        76,
        253,
        50,
        60,
        204,
        178
      ]
    },
    {
      "name": "observerAdded",
      "discriminator": [
        189,
        80,
        125,
        83,
        214,
        120,
        122,
        214
      ]
    },
    {
      "name": "secretChunkAdded",
      "discriminator": [
        164,
        8,
        211,
        161,
        70,
        33,
        25,
        69
      ]
    },
    {
      "name": "vaultClaimed",
      "discriminator": [
        232,
        48,
        41,
        194,
        218,
        163,
        56,
        86
      ]
    },
    {
      "name": "vaultCreated",
      "discriminator": [
        117,
        25,
        120,
        254,
        75,
        236,
        78,
        115
      ]
    },
    {
      "name": "vaultRefunded",
      "discriminator": [
        144,
        207,
        178,
        46,
        176,
        40,
        56,
        210
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "vaultNotFound",
      "msg": "Vault not found"
    },
    {
      "code": 6001,
      "name": "vaultNotLocked",
      "msg": "Vault is not in locked state"
    },
    {
      "code": 6002,
      "name": "conditionsNotMet",
      "msg": "Unlock conditions are not met"
    },
    {
      "code": 6003,
      "name": "invalidUnlockTime",
      "msg": "Invalid unlock time — must be in the future"
    },
    {
      "code": 6004,
      "name": "invalidDeadline",
      "msg": "Invalid deadline — must be after unlock time"
    },
    {
      "code": 6005,
      "name": "invalidRecipient",
      "msg": "Invalid recipient"
    },
    {
      "code": 6006,
      "name": "deadlineNotReached",
      "msg": "Deadline has not been reached yet"
    },
    {
      "code": 6007,
      "name": "claimPeriodExpired",
      "msg": "Claim period has expired (past deadline)"
    },
    {
      "code": 6008,
      "name": "insufficientDeposit",
      "msg": "Insufficient deposit amount"
    },
    {
      "code": 6009,
      "name": "insufficientFee",
      "msg": "Insufficient fee for FHE operations"
    },
    {
      "code": 6010,
      "name": "transferFailed",
      "msg": "SOL transfer failed"
    },
    {
      "code": 6011,
      "name": "notAuthorizedForRefund",
      "msg": "Not authorized for refund"
    },
    {
      "code": 6012,
      "name": "invalidConditionReveal",
      "msg": "Invalid condition reveal (commit-reveal mismatch)"
    },
    {
      "code": 6013,
      "name": "invalidVaultId",
      "msg": "Invalid vault ID (expected next sequential ID)"
    },
    {
      "code": 6014,
      "name": "invalidVaultType",
      "msg": "Vault type not supported for this operation"
    },
    {
      "code": 6015,
      "name": "invalidVaultTypeValue",
      "msg": "Invalid vault type value"
    },
    {
      "code": 6016,
      "name": "creatorOnly",
      "msg": "Only the creator can perform this action"
    },
    {
      "code": 6017,
      "name": "tooManyConditions",
      "msg": "Extra conditions limit reached (max 3 extra)"
    },
    {
      "code": 6018,
      "name": "tooManySecretChunks",
      "msg": "Secret chunks limit reached"
    },
    {
      "code": 6019,
      "name": "nameTooLong",
      "msg": "Name too long (max 32 bytes)"
    },
    {
      "code": 6020,
      "name": "invalidNameLength",
      "msg": "Vault name length exceeds the name buffer"
    },
    {
      "code": 6021,
      "name": "noConditions",
      "msg": "No conditions provided"
    },
    {
      "code": 6022,
      "name": "overflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6023,
      "name": "conditionCountMismatch",
      "msg": "Condition count mismatch during commit verification"
    },
    {
      "code": 6024,
      "name": "unsupportedEncryptedField",
      "msg": "Encrypted field not supported for this vault type"
    },
    {
      "code": 6025,
      "name": "tooManyObservers",
      "msg": "Too many observers (max 10)"
    },
    {
      "code": 6026,
      "name": "observerListFull",
      "msg": "Observer list is full"
    },
    {
      "code": 6027,
      "name": "signatureExpired",
      "msg": "Signature has expired"
    },
    {
      "code": 6028,
      "name": "invalidNonce",
      "msg": "Invalid nonce for activity signature"
    },
    {
      "code": 6029,
      "name": "invalidSignature",
      "msg": "Invalid Ed25519 signature"
    },
    {
      "code": 6030,
      "name": "splTransferFailed",
      "msg": "SPL token transfer failed"
    },
    {
      "code": 6031,
      "name": "invalidExtraConditionAccounts",
      "msg": "Invalid remaining accounts for extra conditions"
    }
  ],
  "types": [
    {
      "name": "activityNonce",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "nonce",
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
      "name": "activityRecorded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "activityTracker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "timestamp",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "conditionDepositTracker",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "token",
            "type": "pubkey"
          },
          {
            "name": "totalAmount",
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
      "name": "conditionDeposited",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "depositor",
            "type": "pubkey"
          },
          {
            "name": "token",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "conditionInput",
      "docs": [
        "Condition input for vault creation (passed as instruction arg)"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "conditionType",
            "type": "u8"
          },
          {
            "name": "value",
            "type": "u64"
          },
          {
            "name": "monitoringAddress",
            "type": "pubkey"
          },
          {
            "name": "tokenAddress",
            "type": "pubkey"
          },
          {
            "name": "valueCommit",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "createVaultArgs",
      "docs": [
        "Args for create_vault instruction (flattened for tx size efficiency)"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "fallbackAddr",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "nameLen",
            "type": "u8"
          },
          {
            "name": "deadline",
            "type": "i64"
          },
          {
            "name": "vaultType",
            "type": "u8"
          },
          {
            "name": "privacyFlags",
            "type": "u8"
          },
          {
            "name": "depositAmount",
            "type": "u64"
          },
          {
            "name": "condition",
            "type": {
              "defined": {
                "name": "conditionInput"
              }
            }
          },
          {
            "name": "recipientCommit",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "fallbackCommit",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "encryptedFieldSet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "fieldType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "euint128",
      "type": {
        "kind": "struct",
        "fields": [
          "u128"
        ]
      }
    },
    {
      "name": "extraCondition",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "conditionType",
            "type": "u8"
          },
          {
            "name": "value",
            "type": "u64"
          },
          {
            "name": "monitoringAddress",
            "type": "pubkey"
          },
          {
            "name": "tokenAddress",
            "type": "pubkey"
          },
          {
            "name": "conditionParam",
            "type": "u64"
          },
          {
            "name": "hasEncryptedValue",
            "type": "bool"
          },
          {
            "name": "encryptedValue",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "valueCommit",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "extraConditionAdded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "conditionType",
            "type": "u8"
          },
          {
            "name": "value",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "observerAdded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "observer",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "observerList",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "observers",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "secretChunk",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "data",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "secretChunkAdded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "index",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "vault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "vaultType",
            "type": "u8"
          },
          {
            "name": "status",
            "type": "u8"
          },
          {
            "name": "privacyFlags",
            "type": "u8"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "nameLen",
            "type": "u8"
          },
          {
            "name": "recipientHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "fallbackHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "recipientPlain",
            "type": "pubkey"
          },
          {
            "name": "fallbackPlain",
            "type": "pubkey"
          },
          {
            "name": "deadline",
            "type": "i64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "depositToken",
            "type": "pubkey"
          },
          {
            "name": "depositAmount",
            "type": "u64"
          },
          {
            "name": "isConfidentialToken",
            "type": "bool"
          },
          {
            "name": "conditionType",
            "type": "u8"
          },
          {
            "name": "unlockValue",
            "type": "u64"
          },
          {
            "name": "monitoringAddress",
            "type": "pubkey"
          },
          {
            "name": "conditionToken",
            "type": "pubkey"
          },
          {
            "name": "conditionParam",
            "type": "u64"
          },
          {
            "name": "hasEncryptedRecipient",
            "type": "bool"
          },
          {
            "name": "hasEncryptedAmount",
            "type": "bool"
          },
          {
            "name": "hasEncryptedName",
            "type": "bool"
          },
          {
            "name": "hasEncryptedConditionValue",
            "type": "bool"
          },
          {
            "name": "hasEncryptedDeposit",
            "type": "bool"
          },
          {
            "name": "hasEncryptedConditionSalt",
            "type": "bool"
          },
          {
            "name": "hasEncryptedFallback",
            "type": "bool"
          },
          {
            "name": "encryptedRecipient",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "encryptedAmount",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "encryptedName",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "encryptedConditionValue",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "encryptedDeposit",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "encryptedConditionSalt",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "encryptedFallback",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "extraConditionsCount",
            "type": "u8"
          },
          {
            "name": "secretChunksCount",
            "type": "u8"
          },
          {
            "name": "conditionCommitsCount",
            "type": "u8"
          },
          {
            "name": "conditionValueCommits",
            "type": {
              "array": [
                {
                  "array": [
                    "u8",
                    32
                  ]
                },
                4
              ]
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "vaultClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "vaultCounter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "vaultCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "vaultType",
            "type": "u8"
          },
          {
            "name": "conditionType",
            "type": "u8"
          },
          {
            "name": "unlockValue",
            "type": "u64"
          },
          {
            "name": "deadline",
            "type": "i64"
          },
          {
            "name": "recipientHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "depositToken",
            "type": "pubkey"
          },
          {
            "name": "depositAmount",
            "type": "u64"
          },
          {
            "name": "conditionToken",
            "type": "pubkey"
          },
          {
            "name": "privacyFlags",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "vaultRefunded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vaultId",
            "type": "u64"
          },
          {
            "name": "refundTo",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
