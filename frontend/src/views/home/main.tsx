const Home = () => {
  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-indigo-900 to-black text-white p-6">
      <header className="text-center mb-16">
        <h1 className="text-5xl font-bold tracking-wide mb-4">Polkalend</h1>
        <p className="text-lg text-indigo-300 max-w-2xl mx-auto">
          A decentralized protocol built on the <strong>Polkadot</strong>{" "}
          blockchain, offering <strong>interest-free loans</strong> using a
          collateralized debt position (CDP) model. Inspired by the{" "}
          <strong>Liquity</strong> protocol.
        </p>
      </header>

      <main className="max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-10">
        {/* Borrow Panel */}
        <section className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 shadow-lg">
          <h2 className="text-2xl font-semibold mb-4">Open a Loan</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-300 mb-1">
                Collateral (DOT)
              </label>
              <input
                type="number"
                className="w-full px-4 py-2 rounded bg-white/10 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500"
                placeholder="Enter amount"
              />
            </div>
            <div>
              <label className="block text-sm text-gray-300 mb-1">
                Loan Amount (PLUSD)
              </label>
              <input
                type="number"
                className="w-full px-4 py-2 rounded bg-white/10 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                placeholder="Enter amount"
              />
            </div>
            <button className="w-full py-2 bg-purple-600 hover:bg-purple-700 rounded-lg font-semibold">
              Open CDP
            </button>
          </div>
        </section>

        {/* Manage Loan Panel */}
        <section className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 shadow-lg">
          <h2 className="text-2xl font-semibold mb-4">Manage Your Loan</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-300 mb-1">
                Repay (PLUSD)
              </label>
              <input
                type="number"
                className="w-full px-4 py-2 rounded bg-white/10 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500"
                placeholder="Amount to repay"
              />
            </div>
            <button className="w-full py-2 bg-pink-600 hover:bg-pink-700 rounded-lg font-semibold">
              Repay Loan
            </button>
            <button className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg font-semibold">
              Withdraw Collateral
            </button>
          </div>
        </section>
      </main>

      <footer className="text-center text-sm text-gray-500 mt-16">
        Built with ❤️ for the Polkadot ecosystem.
      </footer>
    </div>
  );
};

export default Home;
