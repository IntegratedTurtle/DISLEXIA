package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	_ "github.com/mattn/go-sqlite3"
)

type User struct {
	UID      int    `json:"uid"`
	Email    string `json:"email"`
	Password string `json:"password"`
	TrackA   int    `json:"tracka"`
	TrackB   int    `json:"trackb"`
	TrackC   int    `json:"trackc"`
}

type Credentials struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

func initDB() *sql.DB {
	db, err := sql.Open("sqlite3", "./users.db")
	if err != nil {
		log.Fatal(err)
	}

	createTableSQL := `
	CREATE TABLE IF NOT EXISTS users (
		uid INTEGER PRIMARY KEY AUTOINCREMENT,
		email TEXT NOT NULL UNIQUE,
		password TEXT NOT NULL,
		tracka INTEGER NOT NULL,
		trackb INTEGER NOT NULL,
		trackc INTEGER NOT NULL
	);`

	_, err = db.Exec(createTableSQL)
	if err != nil {
		log.Fatalf("Faild to create table: %v", err)
	}

	return db
}

func addUser(db *sql.DB, user User) error {
	insertSQL := `INSERT INTO users (email, password, tracka, trackb, trackc) VALUES (?,?,?,?,?)`
	_, err := db.Exec(insertSQL, user.Email, user.Password, user.TrackA, user.TrackB, user.TrackC)
	return err
}

func getUser(db *sql.DB, credential Credentials) (*User, error) {
	var user User
	querySQL := `SELECT uid, email, password, tracka, trackb, trackc FROM users WHERE email = ? AND password = ?`
	err := db.QueryRow(querySQL, credential.Email, credential.Password).Scan(&user.UID, &user.Email, &user.Password, &user.TrackA, &user.TrackB, &user.TrackC)
	if err != nil {
		return nil, err
	}
	return &user, nil
}

func updateTrack(db *sql.DB, credentials Credentials, track string) error {
	updateSQL := fmt.Sprintf(`UPDATE users SET %s = %s + 1 WHERE email = ? AND password = ?`, track, track)
	_, err := db.Exec(updateSQL, credentials.Email, credentials.Password)
	return err
}

func handleCreateUser(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	var user User
	err := json.NewDecoder(r.Body).Decode(&user)
	// !!! Hier will ich noch cross side scripting prevention einbauen
	if err != nil {
		http.Error(w, "INVALID input", http.StatusBadRequest)
		return
	}

	err = addUser(db, user)
	if err != nil {
		log.Println("Email already exists or invalid data")
		http.Error(w, "Email already exists or invalid data", http.StatusConflict)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func handleGetUser(db *sql.DB, w http.ResponseWriter, r *http.Request) {
	var credential Credentials
	err := json.NewDecoder(r.Body).Decode(&credential)
	//!!! Again here I should add cross side scripting prevention
	if err != nil {
		http.Error(w, "INVALID input", http.StatusBadRequest)
		return
	}

	user, err := getUser(db, credential)
	if err != nil {
		http.Error(w, "User not found or password false", http.StatusUnauthorized)
		return
	}

	json.NewEncoder(w).Encode(user)
}

func handleUpdateTrack(db *sql.DB, track string) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var credentials Credentials
		err := json.NewDecoder(r.Body).Decode(&credentials)
		if err != nil {
			http.Error(w, "Invalid input", http.StatusBadRequest)
			return
		}

		err = updateTrack(db, credentials, track)
		if err != nil {
			http.Error(w, "Update failed or invalid credentials", http.StatusUnauthorized)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}

func main() {
	log.Println("Starting")
	db := initDB()
	defer db.Close()

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case "POST":
			handleCreateUser(db, w, r)
		case "SEARCH":
			handleGetUser(db, w, r)
		default:
			http.Error(w, "Unsupported method", http.StatusMethodNotAllowed)
		}
	})
	http.HandleFunc("/tracka", handleUpdateTrack(db, "tracka"))
	http.HandleFunc("/trackb", handleUpdateTrack(db, "trackb"))
	http.HandleFunc("/trackc", handleUpdateTrack(db, "trackc"))

	log.Println("Server is running on port 8081")
	log.Fatal(http.ListenAndServe(":8081", nil))
}
