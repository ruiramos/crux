import React, { useState, useEffect } from "react";
import init, {
  message as sendMessage,
  response as sendResponse,
  view,
} from "../shared/core";
import * as types from "shared_types/types/shared_types";
import * as bcs from "shared_types/bcs/mod";

interface Message {
  kind: "message";
  message: types.Msg;
}

interface Response {
  kind: "response";
  response: types.Response;
}

function deserializeRequests(bytes: Uint8Array) {
  let deserializer = new bcs.BcsDeserializer(bytes);

  const len = deserializer.deserializeLen();

  let requests = [];

  for (let i = 0; i < len; i++) {
    const request = types.Request.deserialize(deserializer);
    requests.push(request);
  }

  return requests;
}

const useCrux = (init, initialState, requestHandler) => {
  const [initd, setInitd] = useState(false);
  const [state, setState] = useState(initialState);

  useEffect(() => {
    async function run() {
      await init();
      setInitd(true);
      updateState();
    }
    run();
  }, []);

  const updateState = () => {
    const viewDeserializer = new bcs.BcsDeserializer(view());
    const viewModel = types.ViewModel.deserialize(viewDeserializer);
    setState(viewModel);
  };

  const dispatch = (action: Message) => {
    const serializer = new bcs.BcsSerializer();
    action.message.serialize(serializer);
    const requests = sendMessage(serializer.getBytes());
    handleRequests(requests);
  };

  const respond = (action: Response) => {
    const serializer = new bcs.BcsSerializer();
    action.response.serialize(serializer);
    const moreRequests = sendResponse(serializer.getBytes());
    handleRequests(moreRequests);
  };

  const handleRequests = async (bytes: any) => {
    let requests = deserializeRequests(bytes);

    for (const request of requests) {
      switch (request.body.constructor) {
        case types.RequestBodyVariantRender:
          updateState();
          break;
        case types.RequestBodyVariantHttp:
          const url = (request.body as types.RequestBodyVariantHttp).value;

          const resp = await fetch(url);
          const body = await resp.arrayBuffer();
          const response_bytes = Array.from(new Uint8Array(body));

          respond({
            kind: "response",
            response: new types.Response(
              request.uuid,
              new types.ResponseBodyVariantHttp(response_bytes)
            ),
          });
          break;
        default:
          requestHandler(request);
      }
    }
  };

  return [state, dispatch];
};

export default useCrux;
