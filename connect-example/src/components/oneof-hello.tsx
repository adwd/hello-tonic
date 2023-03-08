import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { oneofHello } from '../gen/hello-Greeter_connectquery';

export function OneofHello() {
  const [value, setValue] = useState('');
  const [query, setQuery] = useState('');
  const x = useQuery(oneofHello.useQuery({ message: query }));
  console.log(x.data);
  console.log(x.data?.payload);
  console.log(structuredClone(x.data));
  const result = x.data;

  function renderResult(res: typeof result) {
    if (result == null) {
      return null;
    }

    switch (result.payload.case) {
      case 'i32':
        return <div>i32: {result.payload.value}</div>;

      case 'str':
        return <div>str: {result.payload.value}</div>;
      default:
        break;
    }
  }

  const data = x.data!;

  return (
    <div>
      <label>
        Oneof Hello
        <input type="text" value={value} onChange={(ev) => setValue(ev.target.value)}></input>
      </label>
      <button type="button" onClick={() => setQuery(value)}>
        Send
      </button>
      <div>{renderResult(result)}</div>
      {x.data ? (
        <pre>
          <p>JSON.stringify(data, null, 2)</p>
          <code>{JSON.stringify(data, null, 2)}</code>
          <p>JSON.stringify(structuredClone(data), null, 2)</p>
          <code>{JSON.stringify(structuredClone(data), null, 2)}</code>
          <p>data.toJsonString()</p>
          <code>{data.toJsonString()}</code>
          <p>JSON.stringify(data.toJson(), null, 2)</p>
          <code>{JSON.stringify(data.toJson(), null, 2)}</code>
        </pre>
      ) : null}
    </div>
  );
}
