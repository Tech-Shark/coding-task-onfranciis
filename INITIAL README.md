[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/n8771WqG)

# Data

### Developers Name

# Finance Dapp Coding Challenge

Welcome to the coding challenge! We’re excited to see what you can create. This challenge is designed to evaluate your skills in building decentralized applications (Dapps), handling APIs, and integrating with AI services. The challenge consists of two main tasks:

## Challenge Overview

### 1. Finance Application: Frontend and Backend

Your task is to build a simple decentralized finance application (Dapp) that allows users to input their financial goals and sends the data to an AI service. This will include both the frontend and backend components.

Frontend: Design a user interface where users can input their financial information, including:

```
Purpose (e.g., "Save for a new car")
Timeframe (e.g., "12 months")
Total Amount (e.g., $10,000)
Monthly Income (e.g., $2,500)
Monthly Expenses (e.g., $2,000)

```

Backend: Develop a server that processes the user input and sends it to the AI service in the following JSON format:

```

{
  "purpose": "Save for a new car",
  "timeframe": "12 months",
  "total_amount": 10000,
  "total_income_monthly": 2500,
  "total_expenses_monthly": 2000
}


```

You can use Rust technology stack for this challenge. Ensure that the backend can send the request and handle the response from the AI service.
this is the url.

### endpoint for the simple finance app.

`https://icp-api-budget.fly.dev/budget-plan`

---

### 2. Simple frontend and Canister Code for Fetching Forex Data

As part of this challenge, you will write a canister (smart contract) in Motoko or Rust to fetch data from a Forex API and display it. This requires integrating with any public Forex API to get live exchange rates (e.g., Fixer.io or Open Exchange Rates).

Make sure the Forex data is correctly fetched and displayed within the app.

---

### Additional Resources

For more information on how to build canisters, you can refer to the DFINITY website here: DFINITY https://internetcomputer.org/ - The Internet Computer.

---

Submission

Please upload your code to a GitHub repository and share the link. Ensure that your repository includes:

A README file that explains how to run both the frontend and backend of your app.

Clear documentation of how the canister code works.

We look forward to seeing your submission!

---

## This challenge will give us insight into your full-stack development skills and your ability to integrate with decentralized systems and external APIs.
