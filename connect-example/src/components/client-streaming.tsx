import { createPromiseClient } from '@bufbuild/connect-web';
import { useEffect, useState } from 'react';
import { transport } from '../api/gprc-client';
import { Greeter } from '../gen/hello_connect';
import mitt from 'mitt';
import { PartialMessage } from '@bufbuild/protobuf';
import { HelloRequest } from '../gen/hello_pb';

const emitter = mitt();

function createAsyncIterable() {
  const iter: AsyncIterable<PartialMessage<HelloRequest>> = {
    [Symbol.asyncIterator]() {
      return {
        async next() {
          return new Promise((resolve) => {
            emitter.on('*', (type, e) => {
              console.log({ type, e });
              resolve({ value: e, done: true });
            });
          });
        },
      };
    },
  };

  return iter;
}

export const ClientStreamingHello = () => {
  const [response, setResponse] = useState({});
  const client = useEffect(() => {
    const client = createPromiseClient(Greeter, transport);
    const asyncIterable = createAsyncIterable();
    client.clientStreamingHello(asyncIterable).then((res) => setResponse(res));
  }, []);

  const [value, setValue] = useState('');

  return (
    <div>
      <h2>Client streaming Hello</h2>
      <div>
        <input value={value} onChange={(ev) => setValue(ev.target.value)}></input>
        <button
          onClick={() => {
            emitter.emit('foo', { name: 'yay' });
          }}
        >
          send
        </button>
      </div>
      <div>
        <h2>response</h2>
        <pre>
          <code>{JSON.stringify(response, null, 2)}</code>
        </pre>
      </div>
    </div>
  );
};
