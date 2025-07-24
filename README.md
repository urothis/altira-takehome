# altira-takehome

```
Scheduling Service Take-Home

Background:
At Altira Health, we make calls with AI voice agents. As we scale, we expect to be handling thousands of calls an hour. And each customer may have different requirements for those calls.

We want to see how you reason about scheduling constraints and trade-offs.

What you’re building:
Create a small service that takes a list of patients needing outreach and generates a feasible call schedule. At a minimum, the incoming data will be batches of patient name and phone number.

Requirements:

Timebox to ~2 hours:
Python or Typescript is preferred; anything runnable in Docker is fine.
Feel free to use AI, but be prepared to talk about it
When time is up or you’re satisfied with what you’ve created, share a Github repo with your work including a README

After you submit send an email to ****@altirahealth.com. We’ll review your work and schedule a technical conversation to dive into your choices and next steps.

Good luck and have fun!
```

---

Usage:

```bash
docker compose up
```

* Opens up port 3000 by default

* Endpoint: `/schedule`

* An example payload is located at [examples/payload.json](examples/payload.json)

* I've also included a simple insomnia collection [examples/insomnia.yaml](examples/insomnia.yaml)

* Bearer auth is expected, check out the code for the token (this is bad, but leaving it exposed felt worse)

---

Ingress: endpoint to receive a json payload

```json
[
  {
    "name": "John Doe", // name of the patient
    "phone": "212-555-5555", // assumed to be a valid phone number
    // since we lack a scheduled time here, we would want to define some default behavior for scheduling the call,
    // either we schedule the call for 9am their time zone (for example),
    // or just randomly pick a time within a reasonable range (e.g., 9am to 5pm) to spread out the load on other layers/services
  },
  {
    "name": "Jane Doe",
    "phone": "212-555-5556",
    "scheduled": "2023-04-01T10:00:00Z" // optional field if we can pass in the exact time we want the call/task to be scheduled
  }
]
```

Egress/call schedule:

```json
[
  {
    "name": "John Doe",
    "phone": "212-555-5555",
    "tz": "America/New_York",
    "scheduled": "2023-04-01T10:00:00Z",
    // TODO "scheduled_task_id": "1234567891",
  },
  {
    "name": "Jane Doe",
    "phone": "212-555-5556",
    "tz": "America/New_York",
    "scheduled": "2023-04-01T11:00:00Z",
    // TODO "scheduled_task_id": "1234567890",
  }
]
```

---

Can be invoked from any language that can make http requests or trigger other aws event sources like SQS or step functions.

I wrote this in rust just because it was the fastest for me to output in the time given. But would be easily translatable to other languages if needed.

Main design points:
- we are ingesting a list of patients and we want to extrapolate their time zone based on their phone number (if it's not available) and then return a time that would be acceptable to call in utc(9am-5pm as a simple example).

FUTURE

Thoughts about expanding this service:

My first inclination with bulk data ingestion like this (not long running tasks) is a lambda function that can be triggered by an event source such as an SQS queue or an API Gateway and just return a payload. Data isolated and very clean.

This approach allows for scalability and cost efficiency, as it can handle large volumes of data without the need for a dedicated service running 24/7 and the complexity of scaling.

Following the chain of usage here, it might also make sense to have this same lambda take on an additional responsibility of scheduling the calls/tasks in AWS eventbridge.
https://docs.aws.amazon.com/scheduler/latest/UserGuide/what-is-scheduler.html
At that point we could evoke a ton of different events, such as sending a notification or triggering another lambda function to actually invoke the call functionality at the proper time.

In this expansion, we can add a new field to the return json to return the scheduled task id. This id can be used to modify or cancel the scheduled task.

Would also want to expand functionality to further integrate with eventbridge scheduler to get wider views of the scheduled tasks and other data queries we would want to visualize in UI/dashboards.

One last thought on this would be, we would also want to pass in some additional parameters for retry logic possibly, such as the number of retries and the delay between retries. Eventbridge scheduler also supports retry logic, so we can leverage that feature to handle failed tasks.
