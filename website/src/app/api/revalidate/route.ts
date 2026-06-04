import { revalidatePath } from "next/cache";
import { NextRequest, NextResponse } from "next/server";

/**
 * On-demand revalidation endpoint. Hit by the GitHub `release` webhook
 * (or anything else that needs to bust the cache). Requires the shared
 * secret to match the `REVALIDATE_SECRET` env var configured on Vercel.
 *
 * GitHub webhook config:
 *   Payload URL  : https://<your-domain>/api/revalidate?secret=<REVALIDATE_SECRET>
 *   Content type : application/json
 *   Events       : Releases (published, edited, released)
 */
export const dynamic = "force-dynamic";

async function handle(req: NextRequest) {
  const secret = req.nextUrl.searchParams.get("secret");
  const expected = process.env.REVALIDATE_SECRET;

  if (!expected) {
    return NextResponse.json(
      { ok: false, error: "REVALIDATE_SECRET not configured on the server" },
      { status: 500 }
    );
  }
  if (secret !== expected) {
    return NextResponse.json({ ok: false, error: "bad secret" }, { status: 401 });
  }

  revalidatePath("/");

  return NextResponse.json({
    ok: true,
    revalidated: "/",
    at: new Date().toISOString(),
  });
}

export async function POST(req: NextRequest) {
  return handle(req);
}

// GET is allowed too so you can sanity-check the secret from a browser.
export async function GET(req: NextRequest) {
  return handle(req);
}
