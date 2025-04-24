import "jsr:@supabase/functions-js/edge-runtime.d.ts";
import { createClient } from "jsr:@supabase/supabase-js";

const isValidOS = (os: string) => ["macos"].includes(os);
const isValidArch = (arch: string) => ["aarch64"].includes(arch);
const isValidSemver = (version: string) => !!version?.match(/^\d+\.\d+\.\d+$/);

const SUPABASE_URL = Deno.env.get("SUPABASE_URL")!;
const SUPABASE_ANON_KEY = Deno.env.get("SUPABASE_ANON_KEY")!;

Deno.serve(async (req: Request) => {
  const url = new URL(req.url);
  const parts = url.pathname.split("/").filter(Boolean);
  const [, os, arch, version] = parts;

  if (!isValidOS(os) || !isValidArch(arch) || !isValidSemver(version)) {
    return new Response(
      JSON.stringify({
        error: "Invalid URL parameter",
      }),
      {
        status: 400,
      },
    );
  }

  const supabaseClient = createClient(SUPABASE_URL, SUPABASE_ANON_KEY);

  const { data: releases, error } = await supabaseClient
    .from("releases")
    .select("*")
    .eq("os", os)
    .eq("arch", arch)
    .order("created_at", { ascending: false })
    .limit(1);

  if (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
    });
  }

  const update = (releases || [])[0];

  if (!update || isLatestVersion(version, update.version)) {
    return new Response(null, { status: 204 });
  }

  return new Response(
    JSON.stringify({
      version: update.version,
      pub_date: update.created_at,
      url: `${SUPABASE_URL}/storage/v1/object/public/${update.os}/${update.arch}/${update.version}/Probee.app.tar.gz`,
      signature: update.signature,
      format: "app",
      notes: "These are release notes",
    }),
    {
      headers: {
        "Content-Type": "application/json",
      },
    },
  );
});

function isLatestVersion(userVersion: string, updateVersion: string): boolean {
  const userParts = userVersion.split(".", 3).map(Number);
  const updateParts = updateVersion.split(".", 3).map(Number);

  for (let i = 0; i < Math.max(userParts.length, updateParts.length); i++) {
    const userPart = userParts[i] || 0;
    const updatePart = updateParts[i] || 0;

    if (userPart > updatePart) return true;
    if (userPart < updatePart) return false;
  }

  return true;
}
