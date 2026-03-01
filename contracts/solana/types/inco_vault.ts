/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/inco_token.json`.
 */
export type IncoToken = {
  "address": "3J4yBMqJsp2nURjFcgonJ7XPPh6xj2dJGmXCid5guf9A",
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
      "name": "approve",
      "docs": [
        "Approve a delegate",
        "remaining_accounts: [allowance_account, delegate_address]"
      ],
      "discriminator": [
        69,
        74,
        217,
        36,
        115,
        117,
        97,
        76
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "delegate"
        },
        {
          "name": "owner",
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
      "name": "approveChecked",
      "discriminator": [
        47,
        197,
        254,
        42,
        58,
        201,
        58,
        109
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "mint"
        },
        {
          "name": "delegate"
        },
        {
          "name": "owner",
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
          "name": "ciphertext",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        },
        {
          "name": "decimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "buildMemo",
      "discriminator": [
        1,
        77,
        10,
        59,
        60,
        3,
        84,
        73
      ],
      "accounts": [
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        }
      ],
      "args": [
        {
          "name": "encryptedMemo",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "burn",
      "docs": [
        "Burn tokens",
        "remaining_accounts: [allowance_account, owner_address]"
      ],
      "discriminator": [
        116,
        110,
        29,
        56,
        107,
        219,
        42,
        93
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "authority",
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
      "name": "burnChecked",
      "discriminator": [
        198,
        121,
        200,
        102,
        120,
        208,
        155,
        178
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "authority",
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
          "name": "ciphertext",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        },
        {
          "name": "decimals",
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
      "name": "closeAccount",
      "discriminator": [
        125,
        255,
        149,
        14,
        110,
        34,
        72,
        24
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "closeAccount2022",
      "discriminator": [
        181,
        66,
        187,
        4,
        165,
        230,
        192,
        161
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "create",
      "discriminator": [
        24,
        30,
        200,
        40,
        5,
        28,
        7,
        119
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "associatedToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "wallet"
              },
              {
                "kind": "const",
                "value": [
                  115,
                  26,
                  198,
                  136,
                  123,
                  243,
                  182,
                  169,
                  176,
                  251,
                  128,
                  188,
                  53,
                  145,
                  45,
                  180,
                  7,
                  116,
                  126,
                  175,
                  241,
                  200,
                  71,
                  172,
                  203,
                  234,
                  68,
                  102,
                  88,
                  92,
                  108,
                  62
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "wallet"
        },
        {
          "name": "mint"
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
      "args": []
    },
    {
      "name": "createIdempotent",
      "discriminator": [
        143,
        88,
        34,
        91,
        112,
        20,
        245,
        59
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "associatedToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "wallet"
              },
              {
                "kind": "const",
                "value": [
                  115,
                  26,
                  198,
                  136,
                  123,
                  243,
                  182,
                  169,
                  176,
                  251,
                  128,
                  188,
                  53,
                  145,
                  45,
                  180,
                  7,
                  116,
                  126,
                  175,
                  241,
                  200,
                  71,
                  172,
                  203,
                  234,
                  68,
                  102,
                  88,
                  92,
                  108,
                  62
                ]
              },
              {
                "kind": "account",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "wallet"
        },
        {
          "name": "mint"
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
      "args": []
    },
    {
      "name": "createMasterEdition",
      "discriminator": [
        179,
        210,
        96,
        96,
        57,
        25,
        79,
        69
      ],
      "accounts": [
        {
          "name": "edition",
          "writable": true,
          "signer": true
        },
        {
          "name": "metadata"
        },
        {
          "name": "mint"
        },
        {
          "name": "mintAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "updateAuthority"
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
              "name": "createMasterEditionArgs"
            }
          }
        }
      ]
    },
    {
      "name": "createMetadataAccount",
      "discriminator": [
        75,
        73,
        45,
        178,
        212,
        194,
        127,
        113
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "mintAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "updateAuthority"
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
              "name": "createMetadataArgs"
            }
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
      "name": "freezeAccount",
      "discriminator": [
        253,
        75,
        82,
        133,
        167,
        238,
        43,
        130
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "mint"
        },
        {
          "name": "freezeAuthority",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "grantDecryptionRights",
      "docs": [
        "Grant FHE decryption rights (for secret vaults after conditions met)",
        "remaining_accounts: [allowance, address] × N encrypted fields"
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
      "name": "initializeAccount",
      "discriminator": [
        74,
        115,
        99,
        93,
        197,
        69,
        103,
        7
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "owner"
        },
        {
          "name": "payer",
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
      "args": []
    },
    {
      "name": "initializeAccount3",
      "discriminator": [
        23,
        142,
        140,
        135,
        21,
        160,
        133,
        64
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint"
        },
        {
          "name": "authority",
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
      "args": []
    },
    {
      "name": "initializeMint",
      "discriminator": [
        209,
        42,
        195,
        4,
        129,
        85,
        209,
        44
      ],
      "accounts": [
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
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
          "name": "decimals",
          "type": "u8"
        },
        {
          "name": "mintAuthority",
          "type": "pubkey"
        },
        {
          "name": "freezeAuthority",
          "type": {
            "option": "pubkey"
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
      "name": "mintTo",
      "docs": [
        "Mint tokens to an account",
        "remaining_accounts: [allowance_account, owner_address]"
      ],
      "discriminator": [
        241,
        34,
        48,
        186,
        37,
        179,
        123,
        192
      ],
      "accounts": [
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "mintAuthority",
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
      "name": "mintToChecked",
      "discriminator": [
        229,
        236,
        36,
        240,
        118,
        225,
        45,
        125
      ],
      "accounts": [
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "authority",
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
          "name": "ciphertext",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        },
        {
          "name": "decimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "mintToWithHandle",
      "docs": [
        "Mint tokens using a pre-existing encrypted handle",
        "remaining_accounts: [allowance_account, owner_address]"
      ],
      "discriminator": [
        212,
        0,
        185,
        69,
        107,
        145,
        108,
        149
      ],
      "accounts": [
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "mintAuthority",
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
          "name": "amountHandle",
          "type": {
            "defined": {
              "name": "euint128"
            }
          }
        }
      ]
    },
    {
      "name": "printEdition",
      "discriminator": [
        182,
        213,
        76,
        48,
        196,
        144,
        223,
        103
      ],
      "accounts": [
        {
          "name": "edition",
          "writable": true,
          "signer": true
        },
        {
          "name": "masterEdition",
          "writable": true
        },
        {
          "name": "payer",
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
              "name": "printEditionArgs"
            }
          }
        }
      ]
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
      "name": "refundVault",
      "docs": [
        "Refund vault after deadline"
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
      "name": "removeCreatorVerification",
      "discriminator": [
        41,
        194,
        140,
        217,
        90,
        160,
        139,
        6
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "creator",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "revoke",
      "discriminator": [
        170,
        23,
        31,
        34,
        133,
        173,
        93,
        242
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        }
      ],
      "args": []
    },
    {
      "name": "revoke2022",
      "discriminator": [
        239,
        4,
        176,
        163,
        116,
        155,
        31,
        7
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "incoLightningProgram",
          "address": "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
        }
      ],
      "args": []
    },
    {
      "name": "setAccountOwner",
      "discriminator": [
        188,
        192,
        15,
        103,
        89,
        135,
        159,
        89
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "currentOwner",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newOwner",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "setAndVerifyCollection",
      "discriminator": [
        235,
        242,
        121,
        216,
        158,
        234,
        180,
        234
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "updateAuthority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "collection",
          "type": {
            "defined": {
              "name": "collection"
            }
          }
        }
      ]
    },
    {
      "name": "setCloseAuthority",
      "discriminator": [
        144,
        183,
        224,
        104,
        83,
        213,
        219,
        172
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newAuthority",
          "type": {
            "option": "pubkey"
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
    },
    {
      "name": "setFreezeAuthority",
      "discriminator": [
        159,
        131,
        149,
        192,
        109,
        186,
        68,
        227
      ],
      "accounts": [
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "currentAuthority",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newAuthority",
          "type": {
            "option": "pubkey"
          }
        }
      ]
    },
    {
      "name": "setMintAuthority",
      "discriminator": [
        67,
        127,
        155,
        187,
        100,
        174,
        103,
        121
      ],
      "accounts": [
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "currentAuthority",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newAuthority",
          "type": {
            "option": "pubkey"
          }
        }
      ]
    },
    {
      "name": "signMetadata",
      "discriminator": [
        178,
        245,
        253,
        205,
        236,
        250,
        233,
        209
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "creator",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "thawAccount",
      "discriminator": [
        115,
        152,
        79,
        213,
        213,
        169,
        184,
        35
      ],
      "accounts": [
        {
          "name": "account",
          "writable": true
        },
        {
          "name": "mint"
        },
        {
          "name": "freezeAuthority",
          "writable": true,
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "transfer",
      "docs": [
        "Transfer tokens between accounts",
        "remaining_accounts: [source_allowance, source_owner, dest_allowance, dest_owner]"
      ],
      "discriminator": [
        163,
        52,
        200,
        231,
        140,
        3,
        69,
        186
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "authority",
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
      "name": "transferChecked",
      "discriminator": [
        119,
        250,
        202,
        24,
        253,
        135,
        244,
        121
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "mint"
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "authority",
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
          "name": "ciphertext",
          "type": "bytes"
        },
        {
          "name": "inputType",
          "type": "u8"
        },
        {
          "name": "decimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "transferCheckedWithHandle",
      "docs": [
        "Transfer checked using a pre-existing encrypted handle",
        "remaining_accounts: [source_allowance, source_owner, dest_allowance, dest_owner]"
      ],
      "discriminator": [
        85,
        140,
        71,
        159,
        98,
        189,
        215,
        74
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "mint"
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "authority",
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
          "name": "amountHandle",
          "type": {
            "defined": {
              "name": "euint128"
            }
          }
        },
        {
          "name": "decimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "transferWithHandle",
      "docs": [
        "Transfer tokens using a pre-existing encrypted handle",
        "remaining_accounts: [source_allowance, source_owner, dest_allowance, dest_owner]"
      ],
      "discriminator": [
        93,
        55,
        30,
        205,
        8,
        14,
        246,
        126
      ],
      "accounts": [
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "authority",
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
          "name": "amountHandle",
          "type": {
            "defined": {
              "name": "euint128"
            }
          }
        }
      ]
    },
    {
      "name": "unverifyCollection",
      "discriminator": [
        250,
        251,
        42,
        106,
        41,
        137,
        186,
        168
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "collectionAuthority",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "updateMetadataAccount",
      "discriminator": [
        141,
        14,
        23,
        104,
        247,
        192,
        53,
        173
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "updateAuthority",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": {
              "name": "updateMetadataArgs"
            }
          }
        }
      ]
    },
    {
      "name": "verifyCollection",
      "discriminator": [
        56,
        113,
        101,
        253,
        79,
        55,
        122,
        169
      ],
      "accounts": [
        {
          "name": "metadata",
          "writable": true
        },
        {
          "name": "collectionAuthority",
          "signer": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
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
      "name": "edition",
      "discriminator": [
        234,
        117,
        249,
        74,
        7,
        99,
        235,
        167
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
      "name": "incoAccount",
      "discriminator": [
        18,
        233,
        131,
        18,
        230,
        173,
        249,
        89
      ]
    },
    {
      "name": "incoMint",
      "discriminator": [
        254,
        129,
        245,
        169,
        202,
        143,
        198,
        4
      ]
    },
    {
      "name": "masterEdition",
      "discriminator": [
        58,
        104,
        215,
        125,
        177,
        54,
        116,
        225
      ]
    },
    {
      "name": "metadata",
      "discriminator": [
        72,
        11,
        121,
        26,
        111,
        181,
        85,
        93
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
      "name": "alreadyInitialized",
      "msg": "Metadata account is already initialized"
    },
    {
      "code": 6001,
      "name": "notInitialized",
      "msg": "Metadata account is not initialized"
    },
    {
      "code": 6002,
      "name": "updateAuthorityMismatch",
      "msg": "Update authority mismatch"
    },
    {
      "code": 6003,
      "name": "dataIsImmutable",
      "msg": "Data is immutable"
    },
    {
      "code": 6004,
      "name": "mintMismatch",
      "msg": "Mint mismatch"
    },
    {
      "code": 6005,
      "name": "creatorNotFound",
      "msg": "Creator not found"
    },
    {
      "code": 6006,
      "name": "collectionNotSet",
      "msg": "Collection not set"
    },
    {
      "code": 6007,
      "name": "maxSupplyReached",
      "msg": "Maximum supply reached"
    },
    {
      "code": 6008,
      "name": "numericalOverflow",
      "msg": "Numerical overflow"
    },
    {
      "code": 6009,
      "name": "nameTooLong",
      "msg": "Name too long (max 32 characters)"
    },
    {
      "code": 6010,
      "name": "symbolTooLong",
      "msg": "Symbol too long (max 10 characters)"
    },
    {
      "code": 6011,
      "name": "uriTooLong",
      "msg": "URI too long (max 200 characters)"
    }
  ],
  "types": [
    {
      "name": "accountState",
      "repr": {
        "kind": "rust"
      },
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "uninitialized"
          },
          {
            "name": "initialized"
          },
          {
            "name": "frozen"
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
      "name": "cOption",
      "generics": [
        {
          "kind": "type",
          "name": "t"
        }
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "some",
            "fields": [
              {
                "generic": "t"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "collection",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "key",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "collectionDetails",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "v1",
            "fields": [
              {
                "name": "size",
                "type": "u64"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "collectionDetailsToggle",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "clear"
          },
          {
            "name": "set",
            "fields": [
              {
                "defined": {
                  "name": "collectionDetails"
                }
              }
            ]
          }
        ]
      }
    },
    {
      "name": "collectionToggle",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "clear"
          },
          {
            "name": "set",
            "fields": [
              {
                "defined": {
                  "name": "collection"
                }
              }
            ]
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
      "name": "createMasterEditionArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxSupply",
            "type": {
              "option": "u64"
            }
          }
        ]
      }
    },
    {
      "name": "createMetadataArgs",
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
            "name": "sellerFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "creators",
            "type": {
              "option": {
                "vec": {
                  "defined": {
                    "name": "creator"
                  }
                }
              }
            }
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "collection",
            "type": {
              "option": {
                "defined": {
                  "name": "collection"
                }
              }
            }
          },
          {
            "name": "uses",
            "type": {
              "option": {
                "defined": {
                  "name": "uses"
                }
              }
            }
          },
          {
            "name": "collectionDetails",
            "type": {
              "option": {
                "defined": {
                  "name": "collectionDetails"
                }
              }
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
      "name": "creator",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "verified",
            "type": "bool"
          },
          {
            "name": "share",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "edition",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": {
              "defined": {
                "name": "metadataKey"
              }
            }
          },
          {
            "name": "parent",
            "type": "pubkey"
          },
          {
            "name": "edition",
            "type": "u64"
          },
          {
            "name": "isInitialized",
            "type": "bool"
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
      "name": "incoAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "delegate",
            "type": {
              "defined": {
                "name": "cOption",
                "generics": [
                  {
                    "kind": "type",
                    "type": "pubkey"
                  }
                ]
              }
            }
          },
          {
            "name": "state",
            "type": {
              "defined": {
                "name": "accountState"
              }
            }
          },
          {
            "name": "isNative",
            "type": {
              "defined": {
                "name": "cOption",
                "generics": [
                  {
                    "kind": "type",
                    "type": "u64"
                  }
                ]
              }
            }
          },
          {
            "name": "delegatedAmount",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "closeAuthority",
            "type": {
              "defined": {
                "name": "cOption",
                "generics": [
                  {
                    "kind": "type",
                    "type": "pubkey"
                  }
                ]
              }
            }
          }
        ]
      }
    },
    {
      "name": "incoMint",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintAuthority",
            "type": {
              "defined": {
                "name": "cOption",
                "generics": [
                  {
                    "kind": "type",
                    "type": "pubkey"
                  }
                ]
              }
            }
          },
          {
            "name": "supply",
            "type": {
              "defined": {
                "name": "euint128"
              }
            }
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          },
          {
            "name": "freezeAuthority",
            "type": {
              "defined": {
                "name": "cOption",
                "generics": [
                  {
                    "kind": "type",
                    "type": "pubkey"
                  }
                ]
              }
            }
          }
        ]
      }
    },
    {
      "name": "masterEdition",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": {
              "defined": {
                "name": "metadataKey"
              }
            }
          },
          {
            "name": "supply",
            "type": "u64"
          },
          {
            "name": "maxSupply",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "isInitialized",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "metadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": {
              "defined": {
                "name": "metadataKey"
              }
            }
          },
          {
            "name": "updateAuthority",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
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
            "name": "sellerFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "creators",
            "type": {
              "option": {
                "vec": {
                  "defined": {
                    "name": "creator"
                  }
                }
              }
            }
          },
          {
            "name": "primarySaleHappened",
            "type": "bool"
          },
          {
            "name": "isMutable",
            "type": "bool"
          },
          {
            "name": "editionNonce",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "tokenStandard",
            "type": {
              "option": {
                "defined": {
                  "name": "tokenStandard"
                }
              }
            }
          },
          {
            "name": "collection",
            "type": {
              "option": {
                "defined": {
                  "name": "collection"
                }
              }
            }
          },
          {
            "name": "uses",
            "type": {
              "option": {
                "defined": {
                  "name": "uses"
                }
              }
            }
          },
          {
            "name": "collectionDetails",
            "type": {
              "option": {
                "defined": {
                  "name": "collectionDetails"
                }
              }
            }
          },
          {
            "name": "isInitialized",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "metadataKey",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "uninitialized"
          },
          {
            "name": "metadataV1"
          },
          {
            "name": "editionV1"
          },
          {
            "name": "masterEditionV1"
          },
          {
            "name": "masterEditionV2"
          },
          {
            "name": "editionMarker"
          }
        ]
      }
    },
    {
      "name": "printEditionArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "edition",
            "type": "u64"
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
      "name": "tokenStandard",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "nonFungible"
          },
          {
            "name": "fungibleAsset"
          },
          {
            "name": "fungible"
          },
          {
            "name": "nonFungibleEdition"
          },
          {
            "name": "programmableNonFungible"
          }
        ]
      }
    },
    {
      "name": "updateMetadataArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newUpdateAuthority",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "name",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "symbol",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "uri",
            "type": {
              "option": "string"
            }
          },
          {
            "name": "sellerFeeBasisPoints",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "creators",
            "type": {
              "option": {
                "vec": {
                  "defined": {
                    "name": "creator"
                  }
                }
              }
            }
          },
          {
            "name": "primarySaleHappened",
            "type": {
              "option": "bool"
            }
          },
          {
            "name": "isMutable",
            "type": {
              "option": "bool"
            }
          },
          {
            "name": "collection",
            "type": {
              "defined": {
                "name": "collectionToggle"
              }
            }
          },
          {
            "name": "collectionDetails",
            "type": {
              "defined": {
                "name": "collectionDetailsToggle"
              }
            }
          },
          {
            "name": "uses",
            "type": {
              "defined": {
                "name": "usesToggle"
              }
            }
          }
        ]
      }
    },
    {
      "name": "useMethod",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "burn"
          },
          {
            "name": "multiple"
          },
          {
            "name": "single"
          }
        ]
      }
    },
    {
      "name": "uses",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "useMethod",
            "type": {
              "defined": {
                "name": "useMethod"
              }
            }
          },
          {
            "name": "remaining",
            "type": "u64"
          },
          {
            "name": "total",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "usesToggle",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "clear"
          },
          {
            "name": "set",
            "fields": [
              {
                "defined": {
                  "name": "uses"
                }
              }
            ]
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
/** Alias for backward compatibility */
export type IncoVault = IncoToken;
