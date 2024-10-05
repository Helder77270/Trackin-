import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ModuleConfig from "./config/ModuleConfig";
import HomePage from "./components/pages/HomePage";
import ModulePage from "./components/materials/ModulePage";
import CreatedPage from "./components/pages/CreatedPage";
import AddPage from "./components/pages/AddPage";

const pages = ModuleConfig.getInstance().getConfig().modules.pages;

const router = createBrowserRouter([
    {
        path: pages.Home.props.path,
        element: (
            <ModulePage from={pages.Home}>
                <HomePage />
            </ModulePage>
        ),
    },
    {
        path: pages.Created.props.path,
        element: (
            <ModulePage from={pages.Created}>
                <CreatedPage />
            </ModulePage>
        ),
    },
    {
        path: pages.Add.props.path,
        element: (
            <ModulePage from={pages.Add}>
                <AddPage />
            </ModulePage>
        ),
    },
]);

export default function Routing() {
    return <RouterProvider router={router} />;
}
