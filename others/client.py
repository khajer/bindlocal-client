# This script creates a TCP client that connects to an HTTP server
# and sends a simple GET request.

import socket

# Define the server address and port.
HOST = 'localhost'
PORT = 3000

# Create a socket object using the AF_INET address family (IPv4) and
# SOCK_STREAM socket type (TCP).
client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

try:
    # Connect the socket to the server.
    print(f"Connecting to {HOST}:{PORT}...")
    client_socket.connect((HOST, PORT))

    # Construct the HTTP GET request.
    # The request must end with a blank line (CRLF).
    request = "GET / HTTP/1.1\r\nHost: localhost:3000\r\nConnection: close\r\n\r\n"
    print("Sending HTTP GET request...")

    # Send the encoded request to the server.
    client_socket.sendall(request.encode('utf-8'))

    # Receive data from the server. The buffer size is 4096 bytes.
    print("Receiving response...")
    response = b''
    while True:
        data = client_socket.recv(4096)
        if not data:
            break
        response += data

    # Decode the response and print it.
    print("\n--- Server Response ---")
    print(response.decode('utf-8'))
    print("-----------------------")

except ConnectionRefusedError:
    print(f"Error: Connection refused. Please ensure the server is running on {HOST}:{PORT}.")
except Exception as e:
    print(f"An unexpected error occurred: {e}")

finally:
    # Close the connection. This is in a finally block to ensure the socket
    # is closed even if an error occurs.
    print("Closing the connection.")
    client_socket.close()
