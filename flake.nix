{
	description = " ";

	outputs = { self, crane, fenix, flake-utils, ... } @ inputs: 
		flake-utils.lib.eachDefaultSystem (system: let 
			rust-analyzer = fenix.packages.${system}.stable.rust-analyzer; 
      toolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
						complete.rustfmt
						complete.clippy
            targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

			craneLib = (crane.mkLib inputs.nixpkgs.legacyPackages.${system}).overrideToolchain toolchain;
	
			commonArgs = {
				src = ./.;
				doCheck = false;
			};
	
			cargoArtifacts = craneLib.buildDepsOnly commonArgs; 
		in {
			packages = {
				default = craneLib.buildPackage commonArgs;
				windows = craneLib.buildPackage commonArgs // {
         CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";
				};
			};

			checks = {
				clippy = craneLib.cargoClippy commonArgs // { inherit cargoArtifacts; };
				format = craneLib.cargoFmt commonArgs // { inherit cargoArtifacts; };
			};

			devShells = craneLib.devShell {
				checks = self.checks.${system};
				packages = [ rust-analyzer ];
			};
	});

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

		fenix = {
		  url = "github:nix-community/fenix/monthly";
     	inputs.nixpkgs.follows = "nixpkgs";
		};
	};
}
