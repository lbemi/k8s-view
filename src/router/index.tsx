import { createBrowserRouter } from "react-router-dom";
import GeekLayout from "../pages/Layout";
import { Home } from "../pages/Home";


export const router = createBrowserRouter([
    {
        path: "/",
        element: <Home />

    },{
        path: '/kubernetes',
        element: <GeekLayout/>
    }
])