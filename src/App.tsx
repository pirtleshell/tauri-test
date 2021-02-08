import React from 'react';
import logo from './logo.svg';
import './App.css';
import { useForm } from 'react-hook-form';
import { promisified } from 'tauri/api/tauri';

type State = {
  person: string;
};
const reducer = (state: State, newState: Partial<State>): State => ({
  ...state,
  ...newState,
});

function App() {
  const [state, dispatch] = React.useReducer(reducer, {
    person: 'Buddy',
  });
  const { handleSubmit, register, errors } = useForm({
    defaultValues: { person: '' },
  });

  React.useEffect(() => {
    console.log({ logo });
    promisified({
      cmd: 'doSomething',
      count: 6,
      payload: {
        state: '1',
        data: 10,
      },
    }).then((d) => {
      console.log(d);
    });
  }, []);

  const go = (data: { person: string }): void => {
    dispatch({ person: data.person });
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={`public/${logo}`} className="App-logo" alt="logo" />

        <form
          className="flex items-center space-x-4 p-3"
          onSubmit={handleSubmit(go)}
        >
          <div className="flex flex-col">
            <label htmlFor="person">What's your name?</label>
            <input
              className="text-black"
              ref={register({ required: true })}
              type="text"
              name="person"
            />
            {errors.person && (
              <span className="text-sm text-red-500">
                This field is required.
              </span>
            )}
          </div>
          <button className="flex-grow" type="submit">
            Go
          </button>
        </form>

        <div className="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-md flex items-center space-x-4">
          <div>
            {state.person && (
              <h2 className="text-purple-700">Hey {state.person}!</h2>
            )}
            <p className="text-gray-500 text-sm">
              This app uses TailwindCSS, React, &amp; bundles with Tauri!
            </p>
          </div>
        </div>
      </header>
    </div>
  );
}

export default App;
