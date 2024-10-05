import { useParams } from "react-router-dom";
import PageTemplate from "../PageTemplate";

export default function TestPage() {
    const { seconds } = useParams();
    if (!seconds) return null;

    return <PageTemplate tabTitle="Test"></PageTemplate>;
}
