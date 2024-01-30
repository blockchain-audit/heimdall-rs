#[cfg(test)]
mod benchmark {
    use clap_verbosity_flag::Verbosity;
    use heimdall_common::utils::testing::benchmarks::async_bench;

    use heimdall_core::decompile::DecompilerArgs;

    #[tokio::test]
    async fn benchmark_decompile_solidity_simple() {
        async fn bench() {
            let args = DecompilerArgs {
                target: String::from("0x1bf797219482a29013d804ad96d1c6f84fba4c45"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: true,
                include_yul: false,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            };
            let _ = heimdall_core::decompile::decompile(args).await;
        }

        async_bench("benchmark_decompile_solidity_simple", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_decompile_solidity_complex() {
        async fn bench() {
            let args = DecompilerArgs {
                target: String::from("0xE90d8Fb7B79C8930B5C8891e61c298b412a6e81a"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: true,
                include_yul: false,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            };
            let _ = heimdall_core::decompile::decompile(args).await;
        }

        async_bench("benchmark_decompile_solidity_complex", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_decompile_yul_simple() {
        async fn bench() {
            let args = DecompilerArgs {
                target: String::from("0x1bf797219482a29013d804ad96d1c6f84fba4c45"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: false,
                include_yul: true,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            };
            let _ = heimdall_core::decompile::decompile(args).await;
        }

        async_bench("benchmark_decompile_yul_simple", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_decompile_yul_complex() {
        async fn bench() {
            let args = DecompilerArgs {
                target: String::from("0xE90d8Fb7B79C8930B5C8891e61c298b412a6e81a"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: false,
                include_yul: true,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            };
            let _ = heimdall_core::decompile::decompile(args).await;
        }

        async_bench("benchmark_decompile_yul_complex", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_build_abi_simple() {
        async fn bench() {
            let args = DecompilerArgs {
                target: String::from("0x1bf797219482a29013d804ad96d1c6f84fba4c45"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: false,
                include_yul: false,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            };
            let _ = heimdall_core::decompile::decompile(args).await;
        }

        async_bench("benchmark_build_abi_simple", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_build_abi_complex() {
        async fn bench() {
            let args = DecompilerArgs {
                target: String::from("0xE90d8Fb7B79C8930B5C8891e61c298b412a6e81a"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: false,
                include_yul: false,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            };
            let _ = heimdall_core::decompile::decompile(args).await;
        }

        async_bench("benchmark_build_abi_complex", 100, bench).await;
    }
}

#[cfg(test)]
mod integration_tests {
    use clap_verbosity_flag::Verbosity;
    use heimdall_common::utils::io::file::delete_path;
    use heimdall_core::decompile::DecompilerArgs;

    #[tokio::test]
    async fn test_decompile_precompile() {
        let result = heimdall_core::decompile::decompile(DecompilerArgs {
            target: String::from("0x1bf797219482a29013d804ad96d1c6f84fba4c45"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from("https://eth.llamarpc.com"),
            default: true,
            skip_resolving: true,
            include_solidity: true,
            include_yul: false,
            output: String::from(""),
            name: String::from(""),
            timeout: 10000,
        })
        .await
        .expect("failed to decompile");

        println!("{result:?}");

        // assert that the output is correct
        for line in &["function Unresolved_19045a25(bytes memory arg0, bytes memory arg1) public payable returns (address) {",
            " = ecrecover("] {
            println!("{line}");
            assert!(result.source.clone().expect("decompile source is empty").contains(line));
        }

        // drop path
        delete_path(&String::from("./output/tests/decompile/test1"));
    }

    #[tokio::test]
    async fn test_decompile_weth() {
        let result = heimdall_core::decompile::decompile(DecompilerArgs {
            target: String::from("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from("https://eth.llamarpc.com"),
            default: true,
            skip_resolving: true,
            include_solidity: true,
            include_yul: false,
            output: String::from(""),
            name: String::from(""),
            timeout: 10000,
        })
        .await
        .expect("failed to decompile");

        // assert that the output is correct
        for line in &["function Unresolved_06fdde03() public view returns (bytes memory) {",
            "function Unresolved_095ea7b3(address arg0, bytes memory arg1) public returns (bool) {",
            "function Unresolved_18160ddd() public view returns (address) {",
            "function Unresolved_23b872dd(address arg0, address arg1, bytes memory arg2) public returns (bool) {",
            "function Unresolved_2e1a7d4d(bool arg0) public {",
            "function Unresolved_313ce567() public view returns (bool) {",
            "function Unresolved_70a08231(address arg0) public view returns (uint256) {",
            "function Unresolved_95d89b41() public view returns (bytes memory) {",
            "function Unresolved_a9059cbb(address arg0, bytes memory arg1) public returns (bool) {",
            "function Unresolved_d0e30db0() public payable {",
            "function Unresolved_dd62ed3e(address arg0, address arg1) public view returns (uint256) {"] {
            println!("{line}");
            assert!(result.source.clone().expect("decompile source is empty").contains(line));
        }

        // drop path
        delete_path(&String::from("./output/tests/decompile/test2"));
    }

    #[tokio::test]
    async fn test_decompile_ctf() {
        let result = heimdall_core::decompile::decompile(DecompilerArgs {
            target: String::from("0x9f00c43700bc0000Ff91bE00841F8e04c0495000"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from("https://eth.llamarpc.com"),
            default: true,
            skip_resolving: true,
            include_solidity: true,
            include_yul: false,
            output: String::from(""),
            name: String::from(""),
            timeout: 10000,
        })
        .await
        .expect("failed to decompile");

        // assert that the output is correct
        for line in &["function Unresolved_2fa61cd8(address arg0) public view payable returns (uint16) {",
            "function Unresolved_41161b10(bytes memory arg0, address arg1) public payable returns (bool) {",
            "function Unresolved_06fdde03() public pure payable returns (bytes memory) {"] {
            println!("{line}");
            assert!(result.source.clone().expect("decompile source is empty").contains(line));
        }

        // drop path
        delete_path(&String::from("./output/tests/decompile/test3"));
    }

    /// Thorough testing for decompilation across a large number of contracts
    /// Runs on the top 100 contracts for 2023-06-26
    ///
    /// ## Checks:
    /// - There are no panics or stuck threads
    /// - The indentation and bracket pairing is correct
    /// - The number of opening and closing brackets, parentheses, and curly braces are equal
    /// - The ABI is valid and generated correctly
    /// - There are at least 1 instance of each of the following (on a test basis, not
    ///   per-contract):
    ///   - `function`
    ///   - `event`
    ///   - `require`
    ///   - `error`
    ///  - The ABI matches the solidity outline
    ///  - There is no unreachable code (TODO)
    ///  - There are no empty branches (TODO)
    #[tokio::test]
    #[ignore]
    async fn heavy_test_decompile_thorough() {
        let contracts = [
            "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD",
            "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
            "0xA3C66393049fAB4830C330Dfe658f94A4de0A122",
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
            "0x32400084C286CF3E17e7B677ea9583e60a000324",
            "0x00000000000000ADc04C56Bf30aC9d3c0aAF14dC",
            "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            "0x881D40237659C251811CEC9c364ef91dC08D300C",
            "0x1111111254EEB25477B68fb85Ed929f73A960582",
            "0x6b75d8AF000000e20B7a7DDf000Ba900b4009A80",
            "0xDef1C0ded9bec7F1a1670819833240f027b25EfF",
            "0xaBEA9132b05A70803a4E85094fD0e1800777fBEF",
            "0x6B175474E89094C44Da98b954EedeAC495271d0F",
            "0xae0Ee0A63A2cE6BaeEFFE56e7714FB4EFE48D419",
            "0x1a0ad011913A150f69f6A19DF447A0CfD9551054",
            "0x29469395eAf6f95920E59F858042f0e28D98a20B",
            "0xA69babEF1cA67A37Ffaf7a485DfFF3382056e78C",
            "0x95aD61b0a150d79219dCF64E1E6Cc01f0B64C4cE",
            "0xA9D1e08C7793af67e9d92fe308d5697FB81d3E43",
            "0x3dB52cE065f728011Ac6732222270b3F2360d919",
            "0x000000000000Ad05Ccc4F10045630fb830B95127",
            "0x253553366Da8546fC250F225fe3d25d0C782303b",
            "0x65f2F6Fba44e5AbeFD90C2aEE52B11a243FC7A16",
            "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
            "0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0",
            "0xb0fcB43D3701f00aFD2Fb3d5f577a806F551D2F2",
            "0x0000000000A39bb272e79075ade125fd351887Ac",
            "0xEf1c6E67703c7BD7107eed8303Fbe6EC2554BF6B",
            "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45",
            "0x98C3d3183C4b8A650614ad179A1a98be0a8d6B8E",
            "0x514910771AF9Ca656af840dff83E8264EcF986CA",
            "0x06450dEe7FD2Fb8E39061434BAbCFC05599a6Fb8",
            "0x6982508145454Ce325dDbE47a25d4ec3d2311933",
            "0x2a3DD3EB832aF982ec71669E178424b10Dca2EDe",
            "0xa24787320ede4CC19D800bf87B41Ab9539c4dA9D",
            "0x473037de59cf9484632f4A27B509CFE8d4a31404",
            "0xFD14567eaf9ba941cB8c8a94eEC14831ca7fD1b4",
            "0x4d224452801ACEd8B2F0aebE155379bb5D594381",
            "0xDe30da39c46104798bB5aA3fe8B9e0e1F348163F",
            "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9",
            "0x388C818CA8B9251b393131C08a736A67ccB19297",
            "0x3999D2c5207C06BBC5cf8A6bEa52966cabB76d41",
            "0x3b3ae790Df4F312e745D270119c6052904FB6790",
            "0xB517850510997a34b4DdC8c3797B4F83fAd510c4",
            "0x902F09715B6303d4173037652FA7377e5b98089E",
            "0x5a54fe5234E811466D5366846283323c954310B2",
            "0xd1d2Eb1B1e90B638588728b4130137D262C87cae",
            "0x95e05e2Abbd26943874ac000D87C3D9e115B543c",
            "0x00000000A991C429eE2Ec6df19d40fe0c80088B8",
            "0x737673b5e0a3c68adf4c1a87bca5623cfc537ec3",
            "0x940259178FbF021e625510919BC2FF0B944E5613",
            "0xff612db0583be8d5498731e4e32bc12e08fa6292",
            "0xd5FEa30Ed719693Ec8848Dc7501b582F5de6a5BB",
            "0x4C727a07246A70862e45B2E58fcd82c0eD5Eda85",
            "0x9baa53dD2aB408D9135e549831C06E5c6407bF1d",
        ];

        // define flag checks
        let mut is_function_covered = false;
        let mut is_event_covered = false;
        let mut is_require_covered = false;
        let mut is_error_covered = false;

        for contract in contracts {
            println!("Testing contract: {contract}");
            let result = heimdall_core::decompile::decompile(DecompilerArgs {
                target: contract.to_string(),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from("https://eth.llamarpc.com"),
                default: true,
                skip_resolving: true,
                include_solidity: true,
                include_yul: false,
                output: String::from(""),
                name: String::from(""),
                timeout: 10000,
            })
            .await
            .expect("failed to decompile");

            // assert that the number of opening and closing brackets, parentheses, and curly braces
            // are equal
            let output = result.source.expect("decompile source is empty");
            let open_brackets = output.matches('{').count();
            let close_brackets = output.matches('}').count();
            assert_eq!(open_brackets, close_brackets);

            // let open_parens = output.matches("(").count();
            // let close_parens = output.matches(")").count();
            // assert_eq!(open_parens, close_parens);

            let open_braces = output.matches('[').count();
            let close_braces = output.matches(']').count();
            assert_eq!(open_braces, close_braces);

            // perform flag checks
            if output.contains("function Unresolved_") {
                is_function_covered = true;
            }
            if output.contains("event Event_") {
                is_event_covered = true;
            }
            if output.contains("require(") {
                is_require_covered = true;
            }
            if output.contains("error CustomError_") {
                is_error_covered = true;
            }
        }

        // assert that all flags are true
        assert!(is_function_covered);
        assert!(is_event_covered);
        assert!(is_require_covered);
        assert!(is_error_covered);

        delete_path(&String::from("./output/tests/decompile/integration"));
    }
}
