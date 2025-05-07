import "./App.css";
import NavHeader from "./components/NavHeader";
import Router from "./router";
import { BrowserRouter } from "react-router-dom";
import { ToastContainer } from "react-toastify";
import { UseInkProvider } from "useink";
import { WestendTestnet } from "useink/chains";
import metadata from "./assets/json/polkalend.json";

function App() {
  return (
    <UseInkProvider
      config={{
        dappName: metadata.contract.name,
        chains: [WestendTestnet],
        caller: {
          default: "5EyR7vEk7DtvEWeefGcXXMV6hKwB8Ex5uvjHufm466mbjJkR",
        },
      }}
    >
      <ToastContainer />
      <BrowserRouter>
        <NavHeader />
        <Router />
      </BrowserRouter>
    </UseInkProvider>
  );
}

export default App;
