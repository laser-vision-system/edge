# ========== Base ==========
FROM python:3.13-slim-trixie AS base
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

ENV UV_LINK_MODE=copy \
	UV_COMPILE_BYTECODE=1 \
	PYTHONUNBUFFERED=1

WORKDIR /app

# OpenCV system dependencies.
RUN apt-get update && apt-get install -y --no-install-recommends \
	libgl1 \
	libglib2.0-0 \
	&& rm -rf /var/lib/apt/lists/*


# ========== Development ==========
FROM base AS development

# Copy config files.
COPY pyproject.toml uv.lock .python-version ./

# Install dependencies (including dev).
RUN --mount=type=cache,target=/root/.cache/uv \
	uv sync --frozen --no-install-project

# Copy the entire project (including tests/).
COPY . .

# Final sync to install the local package in editable-like mode.
RUN --mount=type=cache,target=/root/.cache/uv \
	uv sync --frozen

ENV PATH="/app/.venv/bin:$PATH"

CMD ["python", "-m", "edge"]


# ========== Test ==========
# This target only runs tests; if they fail, the docker build fails.
FROM development AS test
RUN uv run pytest


# ========== Production ==========
FROM base AS builder
COPY pyproject.toml uv.lock ./
RUN --mount=type=cache,target=/root/.cache/uv \
	uv sync --frozen --no-install-project --no-dev


FROM base AS production

# Copy only the production venv.
COPY --from=builder /app/.venv /app/.venv

# Copy only necessary files.
COPY src/ ./src/
COPY pyproject.toml uv.lock README.md ./

# Final sync (bytecode compilation happens here).
RUN --mount=type=cache,target=/root/.cache/uv \
	uv sync --frozen --no-dev

ENV PATH="/app/.venv/bin:$PATH"

CMD ["python", "-m", "edge"]
