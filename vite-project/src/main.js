import './style.css';
import { fib } from './fibonacci.js';


document.querySelector('#app').innerHTML = `
  <div>
    <h1>Fibonacci Calculator</h1>
    <div class="card">
      <label for="fibN">Enter n: </label>
      <input type="number" id="fibN" placeholder="e.g., 10" min="0" value="10">
    </div>
    <div class="card">
      <input type="checkbox" id="showLengthOnly" name="showLengthOnly">
      <label for="showLengthOnly">Hex mode</label>
    </div>
    <div class="card">
      <button id="calculateButton" type="button">Calculate F(n)</button>
    </div>
    <div id="resultArea" class="card">
      <p><span id="resultLabel">F(n):</span> <strong id="fibValue">-</strong></p>
      <p>Time Taken: <span id="fibDuration">-</span> ms</p>
    </div>
  </div>
`;

const calculateButton = document.getElementById('calculateButton');
const nInput = document.getElementById('fibN');
const fibValueSpan = document.getElementById('fibValue');
const fibDurationSpan = document.getElementById('fibDuration');
const showLengthOnlyCheckbox = document.getElementById('showLengthOnly');
const resultLabelSpan = document.getElementById('resultLabel');

calculateButton.addEventListener('click', () => {
  const nString = nInput.value;
  const n = parseInt(nString, 10);

  if (isNaN(n) || n < 0) {
    fibValueSpan.textContent = 'Invalid input. Please enter a non-negative integer.';
    fibDurationSpan.textContent = '-';
    resultLabelSpan.textContent = 'F(n):';
    return;
  }

  // Clear previous results
  fibValueSpan.textContent = 'Calculating...';
  fibDurationSpan.textContent = '-';

  // Run the calculation asynchronously
  setTimeout(() => {
    try {
      const { value, duration } = fib(n);
      const showLength = showLengthOnlyCheckbox.checked;

      if (showLength) {
        resultLabelSpan.textContent = 'F(n) in hex:';
        const hex = value.toString("hex");
        fibValueSpan.textContent = hex;
      } else {
        resultLabelSpan.textContent = 'F(n):';
        fibValueSpan.textContent = value.toString();
      }
      fibDurationSpan.textContent = duration.toFixed(3);
    } catch (error) {
      console.error("Error during Fibonacci calculation:", error);
      resultLabelSpan.textContent = 'F(n):';
      fibValueSpan.textContent = 'Error calculating Fibonacci.';
      fibDurationSpan.textContent = '-';
    }
  }, 0);
});


