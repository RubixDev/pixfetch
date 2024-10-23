{
  description = "Pixfetch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }: {
    packages.x86_64-linux.pixfetch = nixpkgs.legacyPackages.x86_64-linux.rustPlatform.buildRustPackage {
      pname = "pixfetch";
      version = "1.0.0";
      src = ./.;
      cargoSha256 = "sha256-hLOzxfSqjelOlrF2xHBIK9ae/lgA5GrEweJzlq7Dve4=";
    };

    defaultPackage.x86_64-linux = self.packages.x86_64-linux.pixfetch;
  };
}
