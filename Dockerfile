FROM python:3.12-alpine AS builder

RUN apk add --no-cache build-base libffi-dev gcc musl-dev

WORKDIR /app

COPY requirements.txt ./
RUN pip install --no-cache-dir --prefix=/install -r requirements.txt

FROM python:3.12-alpine

RUN apk add --no-cache libffi

RUN adduser -D -H appuser && \
    mkdir /app && chown -R appuser /app

COPY --from=builder /install /usr/local

WORKDIR /app
COPY --chown=appuser:appuser . .

USER appuser

EXPOSE 5000

CMD ["gunicorn", "--bind", "0.0.0.0:5000", "run:app"]
