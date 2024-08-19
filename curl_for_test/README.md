Voici une documentation au format Markdown pour le script Bash `user_management.sh` que nous avons créé. Cette documentation décrit les fonctionnalités du script et explique comment l'utiliser.

````markdown
# Documentation pour le Script `user_management.sh`

## Introduction

Ce script Bash permet de gérer les utilisateurs via une API REST. Il offre les fonctionnalités suivantes :

- Création d'un utilisateur
- Modification d'un utilisateur
- Suppression d'un utilisateur

## Prérequis

Avant de pouvoir utiliser ce script, vous devez vous assurer que les éléments suivants sont installés et configurés sur votre système :

- **curl** : Outil en ligne de commande pour effectuer des requêtes HTTP.
- **jq** : Outil en ligne de commande pour manipuler du JSON.

### Installation de `jq`

Sur Ubuntu/Debian :

```bash
sudo apt-get install jq
```
````

Sur macOS (avec Homebrew) :

```bash
brew install jq
```

## Utilisation

### 1. Télécharger et préparer le script

Téléchargez ou copiez le script `user_management.sh` sur votre système. Assurez-vous de lui donner les permissions d'exécution :

```bash
chmod +x user_management.sh
```

### 2. Exécution du script

Pour exécuter le script, utilisez simplement la commande suivante :

```bash
./user_management.sh
```

Le script exécutera les actions suivantes :

1. **Créer un utilisateur** : Le script envoie une requête POST à l'API pour créer un nouvel utilisateur avec les informations spécifiées. L'ID de l'utilisateur créé est extrait de la réponse.
2. **Modifier l'utilisateur** : Après avoir créé l'utilisateur, le script envoie une requête PUT pour mettre à jour certaines informations de l'utilisateur.
3. **Supprimer l'utilisateur** : Enfin, le script envoie une requête DELETE pour supprimer l'utilisateur créé.

### 3. Personnalisation

Vous pouvez personnaliser les données envoyées dans les requêtes en modifiant directement les fonctions `create_user`, `update_user`, et `delete_user` dans le script.

#### Exemple de personnalisation de la création d'un utilisateur :

```bash
create_user() {
  echo "Creating user..."
  response=$(curl -s -X POST $API_URL \
    -H "Content-Type: application/json" \
    -d '{
      "first_name": "Alice",
      "last_name": "Smith",
      "password_hash": "secure_hashed_password",
      "address": "789 Maple St, Springfield",
      "phone_number": "+987654321",
      "ssn": "987-65-4321"
    }')

  echo "Response: $response"
  user_id=$(echo $response | jq -r '.id')
  echo "User created with ID: $user_id"
}
```

## Structure du Script

Le script est composé des sections suivantes :

- **Variables globales** : Contient l'URL de base de l'API.
- **Fonction `create_user`** : Crée un utilisateur en envoyant une requête POST à l'API.
- **Fonction `update_user`** : Met à jour un utilisateur existant en envoyant une requête PUT à l'API.
- **Fonction `delete_user`** : Supprime un utilisateur en envoyant une requête DELETE à l'API.
- **Exécution des fonctions** : Appelle successivement les fonctions pour créer, modifier, et supprimer l'utilisateur.

## Remarques

- **Gestion des erreurs** : Le script actuel ne gère pas explicitement les erreurs HTTP. Pour une utilisation en production, il est recommandé d'ajouter des vérifications supplémentaires pour gérer les échecs de requêtes.

- **Compatibilité** : Le script est compatible avec les systèmes Unix/Linux et macOS.

## Conclusion

Ce script offre un moyen simple de gérer les utilisateurs via une API REST. Vous pouvez l'utiliser tel quel ou le modifier selon vos besoins spécifiques.

# Disclaimer

Je fais ça pour m'entrainer, ça compile mais ne mettez pas ça en production, ça peut servir de base à retravailler en revanche
