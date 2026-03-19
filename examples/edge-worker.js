// FAF Edge MCP - Cloudflare Worker
// Deploy: wrangler deploy

import init, { validate_faf, score_faf } from '../pkg/faf_wasm_sdk.js';

let initialized = false;

async function initWasm() {
  if (!initialized) {
    await init();
    initialized = true;
  }
}

export default {
  async fetch(request, env, ctx) {
    await initWasm();

    const url = new URL(request.url);
    const cors = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type',
    };

    // Handle CORS preflight
    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: cors });
    }

    // Routes
    if (url.pathname === '/score' && request.method === 'POST') {
      return handleScore(request, cors);
    }

    if (url.pathname === '/validate' && request.method === 'POST') {
      return handleValidate(request, cors);
    }

    if (url.pathname === '/health') {
      return new Response(JSON.stringify({ status: 'ok', version: '1.0.0' }), {
        headers: { 'Content-Type': 'application/json', ...cors }
      });
    }

    // API docs
    return new Response(JSON.stringify({
      name: 'FAF Edge MCP API',
      version: '1.0.0',
      endpoints: {
        'POST /score': 'Score FAF content',
        'POST /validate': 'Validate FAF content',
        'GET /health': 'Health check'
      }
    }), {
      headers: { 'Content-Type': 'application/json', ...cors }
    });
  }
};

async function handleScore(request, cors) {
  const start = Date.now();

  try {
    const body = await request.json();
    const content = body.faf_content || body.content;

    if (!content) {
      return new Response(JSON.stringify({ error: 'Missing faf_content' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json', ...cors }
      });
    }

    const faf = new FAF(content);
    const elapsed = Date.now() - start;

    return new Response(JSON.stringify({
      score: {
        weighted: faf.weighted_score,
        truth: faf.truth_score,
        tier: faf.tier,
        completeness: faf.completeness,
        clarity: faf.clarity,
        structure: faf.structure,
        metadata: faf.metadata,
      },
      project: {
        name: faf.name,
        stack: faf.stack,
      },
      latency_ms: elapsed,
    }), {
      headers: { 'Content-Type': 'application/json', ...cors }
    });
  } catch (e) {
    return new Response(JSON.stringify({ error: e.toString() }), {
      status: 400,
      headers: { 'Content-Type': 'application/json', ...cors }
    });
  }
}

async function handleValidate(request, cors) {
  try {
    const body = await request.json();
    const content = body.faf_content || body.content;

    if (!content) {
      return new Response(JSON.stringify({ error: 'Missing faf_content' }), {
        status: 400,
        headers: { 'Content-Type': 'application/json', ...cors }
      });
    }

    const valid = validate_faf(content);

    return new Response(JSON.stringify({ valid }), {
      headers: { 'Content-Type': 'application/json', ...cors }
    });
  } catch (e) {
    return new Response(JSON.stringify({ error: e.toString() }), {
      status: 400,
      headers: { 'Content-Type': 'application/json', ...cors }
    });
  }
}
