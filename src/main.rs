use chrono::{DateTime, Local};
use leptos::{either::Either, leptos_dom::logging::console_log, prelude::*};
use reactive_stores::{Field, OptionStoreExt as _, Store};
use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;

fn main() {
  console_log::init_with_level(log::Level::Debug).unwrap_or_default();
  console_error_panic_hook::set_once();

  mount_to_body(App);
}
#[derive(Store, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct State {
  #[store(key: DateTime<Local> = |note| note.date)]
  pub notes: Vec<Note>,
}

#[derive(Store, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Note {
  pub title: String,
  pub content: String,
  pub date: DateTime<Local>,
}

#[component]
fn App() -> impl IntoView {
  let state = Store::new(State::default());
  let selected_note = Store::new(<Option<Note>>::None);

  if let Some(first_note) = state.notes().get().first() {
    selected_note.set(Some(first_note.to_owned()));
  }
  let add_notes = move |_| {
    let new_note =
      Note { date: Local::now(), title: "Title".to_string(), content: "Content".to_string() };
    state.notes().update(|it| it.insert(0, new_note.clone()));
    selected_note.set(Some(new_note));
  };
  let delete_note = move |remove_index: Signal<usize>, child: Field<Note>| {
    move |event: MouseEvent| {
      event.stop_propagation();
      if selected_note.read().as_ref().is_some_and(|it| it.date == *child.date().read()) {
        match state.notes().read().as_slice() {
          [_single_note] => selected_note.set(None),
          [.., before_last_note, last_note] if last_note.date == *child.date().read() => {
            selected_note.set(Some(before_last_note.to_owned()))
          }
          notes => {
            selected_note.set(
              notes
                .windows(2)
                .find(|window| window[0].date == *child.date().read())
                .map(|window| window[1].to_owned()),
            );
          }
        }
      }
      request_animation_frame(move || {
        state.notes().write().remove(remove_index.get());
      });
    }
  };
  let update_selected_note_title = move |event| {
    state
      .notes()
      .into_iter()
      .filter(|note| note.get().date == selected_note.unwrap().get().date)
      .next()
      .unwrap()
      .title()
      .set(event_target_value(&event));
    selected_note.unwrap().title().set(event_target_value(&event)); // For Updating the selected item in the list
  };
  let update_selected_note_content = move |event| {
    state
      .notes()
      .into_iter()
      .filter(|note| note.get().date == selected_note.unwrap().get().date)
      .next()
      .unwrap()
      .content()
      .set(event_target_value(&event));
    selected_note.unwrap().content().set(event_target_value(&event)); // For Updating the selected item in the list
  };
  view! {
    <div id="main-div">
      <div id="list">
        <button class="add-btn" on:click=add_notes>
          "ADD NEW NOTE"
        </button>
        <ul>
          {move || {
            state
              .notes()
              .into_iter()
              .map(|child| {
                view! {
                  <li
                    class="note-item new-item"
                    class:selected=move || {
                      selected_note.get().is_some_and(|it| { it.date == child.get().date })
                    }
                    on:click=move |_| selected_note.set(Some(child.get()))
                  >
                    <div class="items">
                      <div class="title">{move || format!("{}", child.title().get())}</div>
                      <div class="content">{move || child.content().get()}</div>
                    </div>

                    // on:click=delete_note(index.into(), child.into())
                    <button class="fa fa-trash delete-button"></button>
                  </li>
                }
              })
              .collect_view()
          }}
        </ul>
      </div>

      <div id="editor">
        {move || match selected_note.get() {
          Some(selected_note) => {
            Either::Right(
              view! {
                <textarea
                  id="title-editor"
                  rows="2"
                  prop:value=selected_note.title
                  on:input=update_selected_note_title
                ></textarea>
                <textarea
                  id="content-editor"
                  prop:value=selected_note.content
                  on:input=update_selected_note_content
                ></textarea>
              },
            )
          }
          None => {
            Either::Left(view! { <div style="margin:auto; font-size:21px;">"Pick a note"</div> })
          }
        }}

      </div>
    </div>
  }
}
// #[component]
// pub fn ShowSome<N, T, EF>(
//   children: EF,
//   #[prop(into)] option: Store<Option<T>>,
//   #[prop(optional, into)] fallback: ViewFn,
// ) -> impl IntoView
// where
//   N: IntoView + 'static,
//   T: Clone + 'static + Send + Sync + PartialEq,
//   EF: Fn(T) -> N + 'static + Send + Sync,
// {
//   let memoized_when = Memo::new_owning(move |_| (option.get(), option.with(Option::is_some)));

//   move || match memoized_when.get() {
//     Some(value) => Either::Right(children(value).into_view()),
//     None => Either::Left(fallback.run()),
//   }
// }
