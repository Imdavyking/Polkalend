import "./App.css";
import NavHeader from "./components/NavHeader";
import { WalletProvider } from "./context/WalletContext";
import Router from "./router";
import { BrowserRouter } from "react-router-dom";
import { ToastContainer } from "react-toastify";

function App() {
  return (
    <>
      <WalletProvider>
        <ToastContainer />
        <BrowserRouter>
          <NavHeader />
          <Router />
        </BrowserRouter>
      </WalletProvider>
    </>
  );
}

export default App;
