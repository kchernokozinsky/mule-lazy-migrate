class MuleLazyMigrate < Formula
  desc "Mule 4 migration tool"
  homepage "https://github.com/kchernokozinsky/mule-lazy-migrate"
  url "https://github.com/kchernokozinsky/mule-lazy-migrate/archive/refs/tags/v0.1.2.tar.gz"
  sha256 "cf0ac94c8a97eb30255ccb54f80f23ae25d0b24fcf6d283c0b03e0399b0f481d"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/mule-lazy-migrate", "--help"
  end
end 