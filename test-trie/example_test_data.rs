// Taken from https://github.com/ethereumjs/ethereumjs-monorepo/blob/master/packages/statemanager/test/testdata/verkleKaustinenBlock.json#L1-L2626
// Block number 0x62
pub const PREVIOUS_STATE_ROOT: &str =
    "0x2cf2ab8fed2dcfe2fa77da044ab16393dbdabbc65deea5fdf272107a039f2c61";
pub const EXECUTION_WITNESS_JSON: &str =
    r#"
         {
      "stateDiff": [
        {
          "stem": "0xab8fbede899caa6a95ece66789421c7777983761db3cfb33b5e47ba10f413b",
          "suffixDiffs": [
            {
              "suffix": 97,
              "currentValue": null,
              "newValue": "0x2f08a1461ab75873a0f2d23170f46d3be2ade2a7f4ebf607fc53fb361cf85865"
            }
          ]
        }
      ],
      "verkleProof": {
        "otherStems": [],
        "depthExtensionPresent": "0x12",
        "commitmentsByPath": [
          "0x4900c9eda0b8f9a4ef9a2181ced149c9431b627797ab747ee9747b229579b583",
          "0x491dff71f13c89dac9aea22355478f5cfcf0af841b68e379a90aa77b8894c00e",
          "0x525d67511657d9220031586db9d41663ad592bbafc89bc763273a3c2eb0b19dc"
        ],
        "d": "0x5c6e856174962f2786f0711288c8ddd90b0c317db7769ab3485818460421f08c",
        "ipaProof": {
          "cl": [
            "0x4ff3c1e2a97b6bd0861a2866acecd2fd6d2e5949196429e409bfd4851339832e",
            "0x588cfd2b401c8afd04220310e10f7ccdf1144d2ef9191ee9f72d7d44ad1cf9d0",
            "0x0bb16d917ecdec316d38b92558d46450b21553673f38a824037716bfee067220",
            "0x2bdb51e80b9e43cc5011f4b51877f4d56232ce13035671f191bd4047baa11f3d",
            "0x130f6822a47533ed201f5f15b144648a727217980ca9e86237977b7f0fe8f41e",
            "0x2c4b83ccd0bb8ad8d370ab8308e11c95fb2020a6a62e71c9a1c08de2d32fc9f1",
            "0x4424bec140960c09fc97ee29dad2c3ff467b7e01a19ada43979c55c697b4f583",
            "0x5c8f76533d04c7b868e9d7fcaa901897c5f35b27552c3bf94f01951fae6fcd2a"
          ],
          "cr": [
            "0x31cb234eeff147546cabd033235c8f446812c7f44b597d9580a10bbecac9dd82",
            "0x6945048c033a452d346977ab306df4df653b6e7f3e0b75a705a650427ee30e88",
            "0x38ca3c4ebbee982301b6bafd55bc9e016a7c59af95e9666b56a0680ed1cd0673",
            "0x16160e96b0fb20d0c9c7d9ae76ca9c74300d34e05d3688315c0062204ab0d07b",
            "0x2bc96deadab15bc74546f8882d8b88c54ea0b62b04cb597bf5076fe25c53e43c",
            "0x301e407f62f0d1f6bf56f2e252ca89dd9f3bf09acbb0cca9230ecda24ac783b5",
            "0x3ce1800a2e3f10e641f3ef8a8aaacf6573e9e33f4cb5b429850271528ed3cd31",
            "0x471b1578afbd3f2762654d04db73c6a84e9770f3d6b8a189596fbad38fffa263"
          ],
          "finalEvaluation": "0x07ca48ff9f0fb458967f070c18e5cdf180e93212bf3efba6378384c5703a61fe"
        }
      }
    }
    "#;
