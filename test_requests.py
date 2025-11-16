# generated with Gemini

import requests
import json
import base64

TARGET_URL = 'http://test.127.0.0.1.nip.io/path/to/treasure/'

# Helper function for sending and reporting
def send_request(method, url, data=None, headers=None, content_type=None, label="REQUEST"):
    if content_type:
        headers = headers or {}
        headers['Content-Type'] = content_type
    
    try:
        if method == 'GET':
            response = requests.get(url, headers=headers)
        elif method == 'POST':
            response = requests.post(url, data=data, headers=headers)
        elif method == 'PUT':
            response = requests.put(url, data=data, headers=headers)
        elif method == 'DELETE':
            response = requests.delete(url, headers=headers)
        else:
            print(f"Unknown method: {method}")
            return
            
        print(f"{label} ({method}) Status: {response.status_code} Response: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"{label} ({method}) Error: {e}")

# --- 1. JSON POST (Standard) ---
json_data = json.dumps({"event": "order", "status": "completed"})
send_request('POST', TARGET_URL, data=json_data, content_type='application/json', label="1. JSON POST")

exit()

# --- 2. XML POST (Standard) ---
xml_data = "<root><data>test</data></root>"
send_request('POST', TARGET_URL, data=xml_data, content_type='application/xml', label="2. XML POST")

# --- 3. Form Data POST (application/x-www-form-urlencoded) ---
form_data = 'key1=valueA&key2=valueB'
send_request('POST', TARGET_URL, data=form_data, content_type='application/x-www-form-urlencoded', label="3. Form Data POST")

# --- 4. TEXT Plain POST ---
text_data = "This is a simple plain text message, useful for log payloads."
send_request('POST', TARGET_URL, data=text_data, content_type='text/plain', label="4. TEXT Plain POST")

# --- 5. Custom Vendor Content Type POST ---
# Simulates a system sending a proprietary format
custom_data = 'V1/ID:987/Value:XYZ'
send_request('POST', TARGET_URL, data=custom_data, content_type='application/vnd.mycompany.v1+data', label="5. Vendor Type POST")

# --- 6. Empty Body POST ---
# Checks how the server handles zero-length content
send_request('POST', TARGET_URL, data=None, content_type='application/json', label="6. Empty Body POST")

# --- 7. Request with Custom Headers ---
# Checks if all headers are correctly captured
custom_headers = {
    'X-Request-ID': 'UUID-12345',
    'Authorization': 'Bearer test_token',
    'Cache-Control': 'no-cache'
}
send_request('GET', TARGET_URL, headers=custom_headers, label="7. GET with Custom Headers")

# --- 8. POST with Query Parameters ---
# Checks for separation of query parameters from body
query_url = TARGET_URL + '?source=test&version=2'
send_request('POST', query_url, data='query_body', content_type='text/plain', label="8. POST with Query Params")

# --- 9. Base64 Encoded Body POST (Simulating a binary transfer) ---
# The server should receive the raw encoded string
raw_bytes = b'This payload contains some sensitive information.'
encoded_body = base64.b64encode(raw_bytes).decode('utf-8')
send_request('POST', TARGET_URL, data=encoded_body, content_type='application/base64', label="9. Base64 POST")

# --- 10. PUT Request with Different Path ---
# Checks routing and method handling
put_url = TARGET_URL + '/resource/123'
put_data = json.dumps({"action": "update", "id": 123})
send_request('PUT', put_url, data=put_data, content_type='application/json', label="10. PUT to Resource Path")