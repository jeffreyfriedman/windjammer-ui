# Side-by-side samples

**Everything labelled "Windjammer" is proposed syntax that does not parse today** (see `VISION_2026.md` section 3 and the
compiler requirements R1-R4). The Flutter, Compose, Leptos and SwiftUI samples are written from knowledge of those APIs
(**[K]**, not compiled here); they are idiomatic, not golf. Line counts count non-blank lines of the component only and are
indicative, not a benchmark.

## 1. Counter

Windjammer (6 lines)
```windjammer
component Counter() {
    state count = 0
    view { Button("Clicked ${count} times", on_click: || count += 1) }
}
```
Flutter (about 18 lines)
```dart
class Counter extends StatefulWidget { const Counter({super.key}); @override State<Counter> createState() => _S(); }
class _S extends State<Counter> {
  int count = 0;
  @override Widget build(BuildContext c) =>
    ElevatedButton(onPressed: () => setState(() => count++), child: Text('Clicked $count times'));
}
```
Compose (5 lines)
```kotlin
@Composable fun Counter() {
    var count by remember { mutableIntStateOf(0) }
    Button(onClick = { count++ }) { Text("Clicked $count times") }
}
```
Leptos (6 lines)
```rust
#[component] fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! { <button on:click=move |_| set_count.update(|n| *n += 1)>"Clicked " {count} " times"</button> }
}
```
SwiftUI (5 lines)
```swift
struct Counter: View { @State private var count = 0
  var body: some View { Button("Clicked \(count) times") { count += 1 } } }
```
**Verdict:** Windjammer ties Compose/SwiftUI and beats Flutter and Leptos on ceremony (`move`, tuple signals, `setState`). Not a decisive win: SwiftUI and
Compose are already this short. The difference shows up in cost-per-update (fine-grained, no recomposition/rebuild of the button) and in the lack of per-framework rules.

## 2. Todo list (keyed list, add/toggle/remove, derived count)

Windjammer
```windjammer
struct Todo { id: int, text: string, done: bool }

component TodoApp() {
    state todos: List<Todo> = []
    state draft = ""
    state next_id = 1
    derived remaining = todos.filter(|t| !t.done).len()

    fn add() {
        if draft.trim() == "" { return }
        todos.push(Todo { id: next_id, text: draft, done: false })
        next_id += 1
        draft = ""
    }

    view {
        Column(gap: 8) {
            Row(gap: 8) {
                TextField(bind: draft, placeholder: "What needs doing?", on_submit: add)
                Button("Add", on_click: add)
            }
            for todo in todos key todo.id {
                Row {
                    Checkbox(bind: todo.done, label: todo.text)
                    Button("x", label: "Remove ${todo.text}", on_click: || todos.remove(todo.id))
                }
            }
            Text("${remaining} left")
        }
    }
}
```
(`bind:` is the two-way lowering R5; `for ... key` is a keyed fine-grained list; `todo.done` is a reactive field of a store-like list element. How element
mutation notifies row bindings only, not the whole list, is an open design question, see `VISION_2026.md` 3.10.)

Leptos (same behaviour; note the clones and `move`s)
```rust
#[component] fn TodoApp() -> impl IntoView {
    let todos = RwSignal::new(Vec::<Todo>::new());
    let draft = RwSignal::new(String::new());
    let next_id = RwSignal::new(1);
    let remaining = move || todos.get().iter().filter(|t| !t.done).count();
    let add = move |_| {
        let d = draft.get(); if d.trim().is_empty() { return }
        todos.update(|v| v.push(Todo { id: next_id.get(), text: d, done: false }));
        next_id.update(|n| *n += 1); draft.set(String::new());
    };
    view! {
        <input prop:value=draft on:input:target=move |e| draft.set(e.target().value()) />
        <button on:click=add>"Add"</button>
        <For each=move || todos.get() key=|t| t.id let:todo>
            <label><input type="checkbox" prop:checked=todo.done
                on:change=move |_| todos.update(|v| if let Some(t) = v.iter_mut().find(|t| t.id == todo.id) { t.done = !t.done })/>{todo.text.clone()}</label>
            <button on:click=move |_| todos.update(|v| v.retain(|t| t.id != todo.id))>"x"</button>
        </For>
        <p>{remaining} " left"</p>
    }
}
```
Flutter: `setState` + `ListView.builder(itemBuilder:)` with `ValueKey(todo.id)`, a `TextEditingController` that must be disposed, about 3x the lines. Compose:
`mutableStateListOf<Todo>()` + `LazyColumn { items(todos, key = { it.id }) { ... } }` is about the same length as Windjammer (it is the
strongest competitor on this sample). SwiftUI: `@State var todos: [Todo]`, `ForEach($todos) { $todo in Toggle(todo.text, isOn: $todo.done) }` is also short.
**Verdict:** parity with Compose/SwiftUI on length, clearly shorter than Leptos/Flutter; the real win is that updating one `done` flag touches one binding.

## 3. Async data fetch with loading and error states

Windjammer
```windjammer
component UserCard(id: int) {
    resource user = fetch_json<User>("/api/users/${id}")     // re-fetches when `id` changes, cancels the old request

    view {
        match user {
            Loading   => Spinner(label: "Loading user")
            Failed(e) => ErrorBanner("Could not load: ${e.message}", on_retry: user.reload)
            Ready(u)  => Column { Heading(u.name); Text(u.email) }
        }
    }
}
```
Compose
```kotlin
@Composable fun UserCard(id: Int) {
    val state by produceState<Result<User>?>(null, id) { value = runCatching { api.user(id) } }
    when (val s = state) {
        null -> CircularProgressIndicator()
        else -> s.fold({ Column { Text(it.name); Text(it.email) } }, { Text("Could not load: ${it.message}") /* retry needs more */ })
    }
}
```
Leptos
```rust
#[component] fn UserCard(id: ReadSignal<i32>) -> impl IntoView {
    let user = Resource::new(move || id.get(), |id| async move { fetch_user(id).await });
    view! { <Suspense fallback=|| view!{ <p>"Loading"</p> }>
        {move || user.get().map(|r| match r { Ok(u) => view!{ <h2>{u.name}</h2><p>{u.email}</p> }.into_any(),
                                              Err(e) => view!{ <p>{format!("Could not load: {e}")}</p> }.into_any() })}
    </Suspense> }
}
```
Flutter: `FutureBuilder<User>(future: _future, builder: (ctx, snap) => ...)` with the future cached in `initState`/`didUpdateWidget` to avoid refetching on every rebuild (a classic bug). SwiftUI: `.task(id: id) { ... }` with `@State var state: LoadState`.
**Verdict:** best-in-class if `resource` lands with auto-cancel and typed states; also the hardest to deliver in the interpreter (async unsupported there today, R8).

## 4. Form with validation

Windjammer
```windjammer
form Signup {
    field email: string = validate(required, email_format)
    field age:   int?   = validate(range(13, 120))
    field terms: bool   = validate(must_be_true("Accept the terms"))
}

component SignupPage() {
    let f = Signup.new()
    view {
        Form(f, on_submit: |valid| api.signup(valid)) {      // `valid: Valid<Signup>`; only constructible if all fields pass
            TextField(bind: f.email, label: "Email", keyboard: Email)
            NumberField(bind: f.age, label: "Age (optional)")
            Checkbox(bind: f.terms, label: "I accept the terms")
            Button("Create account", submit: true, enabled: f.valid)
        }
    }
}
```
Errors appear under each field automatically (as accessible descriptions, `aria-describedby` equivalent in the semantic tree), after first blur or submit.

Compose: hand-rolled state holder class with `var email by mutableStateOf("")`, `val emailError get() = ...`, `OutlinedTextField(isError=..., supportingText=...)`.
Flutter: `Form` + `GlobalKey<FormState>` + `TextFormField(validator: (v) => ...)` (idiomatic and about as long, but stringly and imperative: `_key.currentState!.validate()`).
Leptos: no built-in; signals per field and manual error memos (third-party crates exist, quality varies **[K]**).
SwiftUI: no built-in validation framework; `@State` per field and computed `isValid`.
**Verdict:** potential clear win (nobody ships a typed `Valid<T>` form story) but it is language-level new design with no prior art to lean on; treat as M5, not core.

## 5. Animated list (enter, exit, reorder)

Windjammer
```windjammer
for item in items key item.id enter: fade_in(150ms) |> slide_from(Edge::Top)  exit: fade_out(120ms)  move: spring(stiffness: 400, damping: 36) {
    Row(padding: 12) { Text(item.title) }
}
```
Flutter: `AnimatedList` with `GlobalKey<AnimatedListState>` and explicit `insertItem`/`removeItem` calls mirroring your model changes (you must keep two lists in sync).
SwiftUI: `ForEach(items) { ... }.transition(.slide)` inside `withAnimation { items.append(x) }` (concise, mature, Apple-only).
Compose: `LazyColumn { items(items, key = { it.id }) { Row(Modifier.animateItem()) { ... } } }` (concise since 1.7).
**Verdict:** SwiftUI and Compose are already at this level; we match them by making enter/exit/move properties of the keyed list primitive and running them on the compositor. Not a win, a requirement.

## 6. Game HUD (health bar, ammo, toast), rendered in the engine pass

Windjammer (same framework, bound to engine state exposed as signals)
```windjammer
component Hud(player: PlayerState) {          // PlayerState fields are engine-owned signals (read-only here)
    view {
        Overlay(anchor: Corner::BottomLeft, safe_area: true) {
            Column(gap: 4) {
                Bar(value: player.health / player.max_health, color: if player.health < 25 { theme.danger } else { theme.ok }) |> animate(spring())
                Text("${player.ammo} / ${player.reserve}", style: theme.type.mono)
            }
        }
        for toast in player.toasts key toast.id enter: fade_in() exit: fade_out() { Toast(toast.text) }
    }
}
```
Today in the engine (egui immediate mode, from the existing editor path) the same HUD is a per-frame function: `ui.add(ProgressBar::new(hp / max).fill(color))`, state read directly every frame, no transitions,
weak text/IME/a11y. For Bevy UI or Unity UI Toolkit the model is retained nodes + manual system updates.
**Verdict:** the win is not the syntax, it is that the HUD, menus and the editor share the app's widgets, theme, localization and tests, and only changed values cost anything per frame.
