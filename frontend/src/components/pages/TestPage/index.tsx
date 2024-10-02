import { useParams } from "react-router-dom";
import Timer from "../../elements/Timer";
import PageTemplate from "../PageTemplate";

export default function TestPage() {
  const { seconds } = useParams();
  if (!seconds) return null;

  return (
    <PageTemplate tabTitle="Test">
      <Timer seconds={parseInt(seconds)} />
    </PageTemplate>
  );
}
