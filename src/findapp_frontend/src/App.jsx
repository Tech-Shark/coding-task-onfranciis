import { useState } from "react";
import { findapp_backend } from "declarations/findapp_backend";

function App() {
  const [result, setResult] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  function handleSubmit(event) {
    setResult("");
    setError("");
    event.preventDefault();
    const purpose = event.target.elements.purpose.value?.trim();
    const timeframe = event.target.elements.timeframe.value?.trim();
    const totalamount = event.target.elements.totalamount.value?.trim();
    const monthlyincome = event.target.elements.monthlyincome.value?.trim();
    const monthlyexpenses = event.target.elements.monthlyexpenses.value?.trim();

    if (
      [
        purpose,
        timeframe,
        totalamount,
        monthlyincome,
        monthlyexpenses,
      ].includes("")
    ) {
      setError("One of the fields is empty!");
      return false;
    }

    setLoading(true);
    findapp_backend
      .send(purpose, timeframe, totalamount, monthlyincome, monthlyexpenses)
      .then((result) => {
        const parsed = JSON.parse(result);

        if (parsed.detail) {
          setError(parsed.detail);
        } else {
          setResult(parsed);
        }
      })
      .finally(() => {
        setLoading(false);
      });
    return false;
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />

      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="purpose">
          <p>Purpose (e.g., "Save for a new car")</p>
          <input id="purpose" alt="purpose" type="text" />
        </label>

        <label htmlFor="timeframe">
          <p>Timeframe (e.g., "12 months"):</p>
          <input id="timeframe" alt="timeframe" type="text" />
        </label>

        <label htmlFor="totalamount">
          <p>Total Amount (e.g., $10,000):</p>
          <input id="totalamount" alt="totalamount" type="text" />
        </label>

        <label htmlFor="monthlyincome">
          <p>Monthly Income (e.g., $2,500):</p>
          <input id="monthlyincome" alt="monthlyincome" type="text" />
        </label>

        <label htmlFor="monthlyexpenses">
          <p>Monthly Expenses (e.g., $2,000):</p>
          <input id="monthlyexpenses" alt="monthlyexpenses" type="text" />
        </label>

        <button type="submit" disabled={loading}>
          {loading ? "Sending" : " Submit"}
        </button>
      </form>

      <div id="error">{error}</div>

      <section id="result">{result}</section>
    </main>
  );
}

export default App;
