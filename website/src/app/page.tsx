import Site from "./Site";
import { getReleaseAssets } from "@/lib/release";

export default async function Page() {
  const assets = await getReleaseAssets();
  return <Site assets={assets} />;
}
