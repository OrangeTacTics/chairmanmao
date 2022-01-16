import requests
import os
from dotenv import load_dotenv
import json

load_dotenv()
JWT = os.environ['JWT']

headers = {
    'Authorization': f"BEARER {JWT}",
    "Content-Type": "application/json",
}

data = json.dumps({
    "query": """
query {
  exams {
  	name
    numQuestions
    maxWrong
    timelimit
    hskLevel
    deck {
      question
      validAnswers
      meaning
    }
  }
}""",
})

response = requests.post(
    "https://dailymandarinthread.info/graphql",
    headers=headers,
    data=data,
)

print(json.dumps(response.json(), indent=4, ensure_ascii=False))
