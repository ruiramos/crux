import React, { useState, useEffect } from "react";
import Head from "next/head";
import init, {
  message as sendMessage,
  response as sendResponse,
  view,
} from "../shared/core";
import * as types from "shared_types/types/shared_types";
import * as bcs from "shared_types/bcs/mod";
import useCrux from "../hooks/useCrux";

interface Todo {
  content: string;
  state: "active" | "completed";
}

const Todo = ({ content, state, editTodo }) => {
  const toggleState = () => {
    editTodo({ content, state: state === "active" ? "completed" : "active" });
  };

  return (
    <div>
      <input
        type="checkbox"
        checked={state === "completed"}
        onChange={toggleState}
      />
      <input
        type="text"
        value={content}
        onChange={(e) => editTodo({ state, content: e.target.value })}
      />
    </div>
  );
};

const Index = () => {
  const [newTodoText, setNewTodoText] = useState<string>("");
  const [todos, setTodos] = useState<Todo[]>([]);
  const [filter, setFilter] = useState<Todo["state"]>();
  const [state, dispatch] = useCrux(
    init,
    { todos: [] },
    (request: types.Request) => {
      switch (request.body.constructor) {
        case types.RequestBodyVariantKVRead:
          console.log(request);
          break;
        case types.RequestBodyVariantKVWrite:
          console.log(request);
          let deserializer = new bcs.BcsDeserializer(request.body.field1);
          const body = types.Model.deserialize(deserializer);

          console.log(body);
          break;
      }
    }
  );

  const handleAddTodo = (e) => {
    e.preventDefault();
    setNewTodoText("");

    dispatch({
      kind: "message",
      message: new types.MsgVariantAddTodo(
        new types.Todo(newTodoText, new types.TodoStateVariantActive())
      ),
    });
  };

  const handleEditTodo =
    (i) =>
    ({ content, state }) => {
      dispatch({
        kind: "message",
        message: new types.MsgVariantUpdateTodo(
          i,
          new types.Todo(
            content,
            state === "active"
              ? new types.TodoStateVariantActive()
              : new types.TodoStateVariantCompleted()
          )
        ),
      });
    };

  const allTodos = state.todos.map((t) => ({
    content: t.content,
    state:
      t.state instanceof types.TodoStateVariantActive ? "active" : "completed",
  }));
  const shownTodos = allTodos.filter((t) => !filter || filter === t.state);

  return (
    <>
      <form onSubmit={handleAddTodo}>
        <input
          type="text"
          value={newTodoText}
          onChange={(e) => setNewTodoText(e.target.value)}
        />
      </form>
      {shownTodos.map((t, i) => (
        <Todo
          key={`todo-${allTodos.indexOf(t)}`}
          editTodo={handleEditTodo(allTodos.indexOf(t))}
          {...t}
        />
      ))}
      Show all{" "}
      <input
        type="radio"
        name="filter"
        onChange={() => setFilter(null)}
        checked={!filter}
      />
      Show active{" "}
      <input
        type="radio"
        name="filter"
        onChange={() => setFilter("active")}
        checked={filter === "active"}
      />
      Show completed{" "}
      <input
        type="radio"
        name="filter"
        onChange={() => setFilter("completed")}
        checked={filter === "completed"}
      />
    </>
  );
};

export default Index;
