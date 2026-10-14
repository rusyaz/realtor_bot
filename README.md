# realtor_bot

Telegram-бот для приёма заявок от собственников, которые хотят сдать квартиру
(Москва и МО). Заявка (анкета + фото) приходит риэлтору в личку.

## Как работает

- **Long polling** — бот сам ходит в `api.telegram.org`.
  Домен, HTTPS, открытые порты и reverse proxy **не нужны**.
- Базы данных нет: состояние анкет хранится в памяти (`InMemStorage`).
  При рестарте недозаполненные анкеты теряются — деплоить лучше ночью.
- Нужен только исходящий доступ к `api.telegram.org`.

## Переменные окружения

| Переменная | Что это |
|---|---|
| `TELOXIDE_TOKEN` | токен бота от @BotFather |
| `REALTOR_CHAT_ID` | chat_id риэлтора — куда слать заявки |
| `SUPPORT_USERNAME` | ник для кнопки «Поддержка», без `@` |
| `RUST_LOG` | уровень логов, обычно `info` |

Шаблон — `.env.example`.

### Как узнать `REALTOR_CHAT_ID`

1. Запустить бота с любым `REALTOR_CHAT_ID` (например `0`).
2. Риэлтор пишет боту `/start`.
3. В логах появится строка `message from chat_id=123456789` — это он.
4. Вписать в `.env`, перезапустить.

Без шага 2 бот не сможет писать риэлтору: Telegram запрещает ботам писать
первыми.

## Запуск

Локально:

```
cp .env.example .env   # заполнить
RUST_LOG=info cargo run
```

На сервере — один из вариантов:

- **systemd**: `cargo build --release`, бинарник и `.env` положить в
  `/opt/realtor_bot/`, юнит — `deploy/realtor_bot.service`.
- **Docker** (как Lowly): отдельный контейнер, `--restart=always`,
  `--env-file .env`, без `-p` — порты не нужны.

## Структура кода

```
src/
├── main.rs          запуск
├── config.rs        чтение .env
├── state.rs         шаги анкеты (State) и заявка (Lead)
├── texts.rs         все тексты бота
├── keyboards.rs     клавиатуры
├── notify.rs        отправка заявки риэлтору
└── handlers/        обработчики: mod.rs — роутинг state → хендлер
```

Поток: `MainMenu → Owner → Region → Address → Floor → Area → Rooms → Repair
→ Photos → Phone → MainMenu`.
