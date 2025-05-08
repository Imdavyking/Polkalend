import { useRoutes } from "react-router-dom";
import Home from "../views/home/main";
import NotFound from "../views/not-found/main";
import CreateLoan from "../views/create-loan/main";
import AcceptLoanForm from "../views/accept-loan/main";
function Router() {
  const routes = [
    {
      path: "/",
      element: <Home />,
    },
    {
      path: "*",
      element: <NotFound />,
    },
    {
      path: "/create-loan",
      element: <CreateLoan />,
    },
    {
      path: "/accept-loan",
      element: <AcceptLoanForm />,
    },
  ];
  return useRoutes(routes);
}

export default Router;
