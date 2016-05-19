'use strict';

var dbm;
var type;
var seed;
var fs = require('fs');
var path = require('path');
var Promise;

/**
  * We receive the dbmigrate dependency from dbmigrate initially.
  * This enables us to not have to rely on NODE_PATH.
  */
exports.setup = function(options, seedLink) {
  dbm = options.dbmigrate;
  type = dbm.dataType;
  seed = seedLink;
  Promise = options.Promise;
};

exports.up = function(db) {
  var filePath = path.join(__dirname + '/sqls/20160519040119-create-items-up.sql');
  return db.runSql(fs.readFileSync(filePath, 'utf8').replace(/\r?\n|\r/gm, ''));
};

exports.down = function(db) {
  var filePath = path.join(__dirname + '/sqls/20160519040119-create-items-down.sql');
  return db.runSql(fs.readFileSync(filePath, 'utf8').replace(/\r?\n|\r/gm, ''));
};
