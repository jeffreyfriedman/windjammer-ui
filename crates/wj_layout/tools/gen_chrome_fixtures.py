#!/usr/bin/env python3
"""Record flexbox layouts from headless Chrome as test fixtures (an implementation independent of Taffy).

Usage: tools/gen_chrome_fixtures.py [--count N] [--seed S] [--chrome PATH]
Writes tests/fixtures/chrome_flex.json: a list of {spec, rects} where `spec` is a tree in the scene's
vocabulary and `rects` the border boxes Chrome computed, relative to each node's parent, in pre-order.
Fixtures are committed; regenerate only to widen coverage, and review the diff.
"""
import argparse, json, os, random, subprocess, sys, tempfile

DIRS = ["row", "column"]
ALIGN = {"start": "flex-start", "center": "center", "end": "flex-end", "stretch": "stretch"}
JUST = {"start": "flex-start", "center": "center", "end": "flex-end", "space_between": "space-between", "space_around": "space-around"}

def gen(rng, depth):
    n = {
        "direction": rng.choice(DIRS),
        "width": rng.choice([None, None, ("px", rng.randint(5, 80)), ("pct", rng.choice([25, 50, 100]))]) ,
        "height": rng.choice([None, None, ("px", rng.randint(5, 60))]),
        "padding": rng.choice([0, 0, rng.randint(1, 8)]),
        "margin": [rng.choice([0, 0, rng.randint(1, 6)]) for _ in range(4)],
        "gap": rng.choice([0, 0, rng.randint(1, 6)]),
        "grow": rng.choice([0, 0, 1, 2]),
        "shrink": rng.choice([0, 1, 1]),
        "align": rng.choice(list(ALIGN)),
        "justify": rng.choice(list(JUST)),
        "children": [],
    }
    if depth > 0:
        n["children"] = [gen(rng, depth - 1) for _ in range(rng.randint(0, 4))]
    return n

def css(n, root=False):
    def dim(d):
        if d is None: return "auto"
        return f"{d[1]}px" if d[0] == "px" else f"{d[1]}%"
    m = n["margin"]
    s = (f"display:flex;box-sizing:border-box;position:relative;flex-direction:{n['direction']};"
         f"width:{dim(n['width'])};height:{dim(n['height'])};padding:{n['padding']}px;"
         f"margin:{m[1]}px {m[2]}px {m[3]}px {m[0]}px;gap:{n['gap']}px;flex-grow:{n['grow']};flex-shrink:{n['shrink']};"
         f"align-items:{ALIGN[n['align']]};justify-content:{JUST[n['justify']]};")
    return s

def html(n, vp):
    def rec(n, root=False):
        st = css(n)
        if root:
            st = st.replace("margin:", "margin:0px;x-m:") + f"width:{vp[0]}px;height:{vp[1]}px;"
        inner = "".join(rec(c) for c in n["children"])
        return f'<div class="n" style="{st}">{inner}</div>'
    return rec(n, True)

PAGE = """<!doctype html><html><head><style>html,body{margin:0;padding:0}.wrap{position:absolute;left:0;top:0}</style></head>
<body>%s<pre id="out"></pre>
<script>
function walk(e,p,out){const r=e.getBoundingClientRect();const pr=p?p.getBoundingClientRect():{left:0,top:0};
out.push([r.left-pr.left,r.top-pr.top,r.width,r.height]);for(const c of e.children){if(c.classList.contains('n'))walk(c,e,out);}}
const all=[];for(const w of document.querySelectorAll('.wrap')){const out=[];walk(w.firstElementChild,null,out);all.push(out);}
document.getElementById('out').textContent=JSON.stringify(all);
</script></body></html>"""

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--count", type=int, default=200)
    ap.add_argument("--seed", type=int, default=20261019)
    ap.add_argument("--chrome", default="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    a = ap.parse_args()
    rng = random.Random(a.seed)
    trees = []
    body = ""
    for i in range(a.count):
        vp = [rng.randint(100, 400), rng.randint(100, 400)]
        tree = gen(rng, rng.randint(1, 3))
        trees.append((vp, tree))
        body += '<div class="wrap">' + html(tree, vp) + "</div>"
    tmp = tempfile.mkdtemp(prefix="wj-chrome-")
    path = os.path.join(tmp, "all.html")
    open(path, "w").write(PAGE % body)
    res = subprocess.run([a.chrome, "--headless=new", "--disable-gpu", "--no-sandbox", f"--user-data-dir={tmp}/profile",
                          "--window-size=800,800", "--dump-dom", "file://" + path], capture_output=True, text=True, timeout=300)
    dom = res.stdout
    k = dom.find('<pre id="out">')
    if k < 0:
        sys.exit("no output from chrome: " + res.stderr[:300])
    all_rects = json.loads(dom[k + len('<pre id="out">'):dom.find("</pre>", k)])
    fixtures = [{"viewport": vp, "spec": tree, "rects": r} for (vp, tree), r in zip(trees, all_rects)]
    here = os.path.dirname(os.path.abspath(__file__))
    outp = os.path.join(here, "..", "tests", "fixtures", "chrome_flex.json")
    json.dump(fixtures, open(outp, "w"), separators=(",", ":"))
    print("wrote", len(fixtures), "fixtures to", outp)

main()
