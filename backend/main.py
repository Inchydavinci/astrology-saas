from fastapi import FastAPI, APIRouter

app = FastAPI()

# ✅ Create an API router with the '/api' prefix
api_router = APIRouter(prefix="/api", tags=["API Routes"])

# ✅ Health check endpoint
@api_router.get("/health", tags=["Health Check"])
def health_check():
    return {"status": "ok"}

# ✅ Include the router in the FastAPI app
app.include_router(api_router)

# ✅ Root endpoint
@app.get("/", tags=["Root"])
def read_root():
    return {"message": "Welcome to the Astrology SaaS Backend!"}
