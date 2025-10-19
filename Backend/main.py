from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def root():
    return {"message": "Hello, PersonaVault!. FastAPI is running securely behind rust layer."}