import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ModuleConfig from "./config/ModuleConfig";
import HomePage from "./components/pages/HomePage";
import ModulePage from "./components/materials/ModulePage";
import TestPage from "./components/pages/TestPage";

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
        path: pages.Test.props.path,
        element: (
            <ModulePage from={pages.Test}>
                <TestPage />
            </ModulePage>
        ),
    },
]);

export default function Routing() {
    return <RouterProvider router={router} />;
}
