# TEST FIXTURE ONLY — Factory security-review test (FT-3b). Do not merge.
import subprocess

API_KEY = "sk-test-1234567890abcdef"

def run(user_input):
    subprocess.run(user_input, shell=True)
