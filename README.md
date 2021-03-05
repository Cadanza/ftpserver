# Serveur FTP en Rust

Saulquin Clément/Aurélie
début du projet : 02/02/2021

# WARNING

Le code a été tester en utilisant la commande **ftp** founis par linux. Tous le code créer jusuq'a présent fonctionne avec cette commande mais beaucoup de bug sont présent si on utilise le logiciel Filezilla!!

## Compilation du projet avec cargo

Lancer le script bash **install.sh** qui permet de compiler le programmer et de généré la doccumentation du programme.

## Lancement du programme

Une fois le script **install.sh** lancer, pour lancer le programme du serveur, il ffaut éxecuter la commande suivante : 
` cargo r path port` avec:
- path : le chemin du dossier auquel les clients auront accés
- port : le port TCP de votre serveur


